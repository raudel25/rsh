use super::Shell;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{copy, Read};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::path::Path;
use std::process::exit;
use std::process::{Command, Stdio};

#[derive(PartialEq)]
pub enum Redirect {
    In,
    Out,
    OutAppend,
}

#[derive(PartialEq)]
pub enum Chain {
    Multiple,
    And,
    Or,
}

pub enum Special {
    True,
    False,
    Exit,
}

pub trait Execute {
    fn execute(&self, shell: &mut Shell, stdin: i32, out: bool) -> (i32, bool);
}

pub struct CommandSystem<'a> {
    program: &'a str,
    args: &'a [&'a str],
}

impl CommandSystem<'_> {
    pub fn new<'a>(program: &'a str, args: &'a [&'a str]) -> CommandSystem<'a> {
        CommandSystem { program, args }
    }
}

impl Execute for CommandSystem<'_> {
    fn execute(&self, _: &mut Shell, stdin: i32, out: bool) -> (i32, bool) {
        let stdin = if stdin == -1 {
            Stdio::inherit()
        } else {
            unsafe { Stdio::from_raw_fd(stdin) }
        };
        let stdout = if out {
            Stdio::inherit()
        } else {
            Stdio::piped()
        };

        let command = Command::new(&self.program)
            .args(self.args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn();

        match command {
            Ok(mut command) => {
                match command.wait() {
                    Ok(status) => {
                        if !out {
                            return (
                                command.stdout.take().unwrap().into_raw_fd(),
                                status.success(),
                            );
                        }

                        return (-1, status.success());
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
            Err(e) => eprintln!("{}", e),
        };

        (-1, false)
    }
}

pub struct Cd<'a> {
    args: &'a [&'a str],
}

impl Cd<'_> {
    pub fn new<'a>(args: &'a [&'a str]) -> Cd<'a> {
        Cd { args }
    }
}

impl Execute for Cd<'_> {
    fn execute(&self, _: &mut Shell, _: i32, _: bool) -> (i32, bool) {
        let new_dir = self.args[1];

        let root = Path::new(new_dir);

        if let Err(e) = env::set_current_dir(&root) {
            eprintln!("{}", e);

            return (-1, false);
        }

        (-1, true)
    }
}

pub struct Pipe<'a> {
    command1: Box<dyn Execute + 'a>,
    command2: Box<dyn Execute + 'a>,
}

impl Pipe<'_> {
    pub fn new<'a>(command1: Box<dyn Execute + 'a>, command2: Box<dyn Execute + 'a>) -> Pipe<'a> {
        Pipe { command1, command2 }
    }
}

impl Execute for Pipe<'_> {
    fn execute(&self, shell: &mut Shell, stdin: i32, out: bool) -> (i32, bool) {
        let (stdout, status1) = self.command1.execute(shell, stdin, false);
        let (stdout, status2) = self.command2.execute(shell, stdout, out);

        (stdout, status1 && status2)
    }
}

pub struct RedirectCommand<'a> {
    command: Box<dyn Execute + 'a>,
    redirect: Redirect,
    path: &'a str,
}

impl RedirectCommand<'_> {
    pub fn new<'a>(
        command: Box<dyn Execute + 'a>,
        redirect: Redirect,
        path: &'a str,
    ) -> RedirectCommand<'a> {
        RedirectCommand {
            command,
            redirect,
            path,
        }
    }
}

impl Execute for RedirectCommand<'_> {
    fn execute(&self, shell: &mut Shell, stdin: i32, out: bool) -> (i32, bool) {
        let file = match self.redirect {
            Redirect::In => File::open(self.path),
            Redirect::Out => OpenOptions::new().write(true).create(true).open(self.path),
            Redirect::OutAppend => OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(self.path),
        };

        match file {
            Ok(mut file) => {
                let stdin = if self.redirect == Redirect::In {
                    file.as_raw_fd()
                } else {
                    stdin
                };

                let out = if self.redirect == Redirect::In {
                    out
                } else {
                    false
                };

                let (stdout, status) = self.command.execute(shell, stdin, out);

                if self.redirect == Redirect::In || stdout == -1 {
                    return (stdout, status);
                }

                let mut out_file = unsafe { File::from_raw_fd(stdout) };

                match copy(&mut out_file, &mut file) {
                    Ok(_) => {
                        return (-1, status);
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
            Err(e) => eprintln!("{}", e),
        }

        (-1, false)
    }
}

pub struct ChainCommand<'a> {
    command1: Box<dyn Execute + 'a>,
    command2: Box<dyn Execute + 'a>,
    chain: Chain,
}

impl ChainCommand<'_> {
    pub fn new<'a>(
        command1: Box<dyn Execute + 'a>,
        command2: Box<dyn Execute + 'a>,
        chain: Chain,
    ) -> ChainCommand<'a> {
        ChainCommand {
            command1,
            command2,
            chain,
        }
    }
}

impl Execute for ChainCommand<'_> {
    fn execute(&self, shell: &mut Shell, _: i32, out: bool) -> (i32, bool) {
        let (stdout1, status1) = self.command1.execute(shell, -1, out);
        let (mut stdout2, mut status2) = (-1, true);

        match self.chain {
            Chain::And => {
                if status1 {
                    (stdout2, status2) = self.command2.execute(shell, -1, out);
                }
            }
            Chain::Or => {
                if !status1 {
                    (stdout2, status2) = self.command2.execute(shell, -1, out);
                }
            }
            Chain::Multiple => {
                (stdout2, status2) = self.command2.execute(shell, -1, out);
            }
        };

        let out1 = if stdout1 != -1 {
            fd_to_str(stdout1)
        } else {
            String::from("")
        };
        let out2 = if stdout1 != -1 {
            fd_to_str(stdout2)
        } else {
            String::from("")
        };

        let mut result = String::new();
        result.push_str(&out1);
        result.push_str(&out2.trim());

        return (str_to_fd(&result, shell), status1 && status2);
    }
}

pub struct SpecialCommand {
    special: Special,
}

impl SpecialCommand {
    pub fn new(special: Special) -> SpecialCommand {
        SpecialCommand { special }
    }
}

impl Execute for SpecialCommand {
    fn execute(&self, _: &mut Shell, _: i32, _: bool) -> (i32, bool) {
        match self.special {
            Special::True => (-1, true),
            Special::False => (-1, false),
            Special::Exit => exit(1),
        }
    }
}

pub struct GetSet<'a> {
    args: &'a [&'a str],
    get: bool,
}

impl GetSet<'_> {
    pub fn new<'a>(args: &'a [&'a str], get: bool) -> GetSet<'a> {
        GetSet { args, get }
    }
}

impl Execute for GetSet<'_> {
    fn execute(&self, shell: &mut Shell, _: i32, out: bool) -> (i32, bool) {
        let mut stdout = String::new();
        let mut status = true;

        if !self.get {
            if self.args.len() == 3 {
                let _ = &shell
                    .variables
                    .insert(self.args[1].to_string(), self.args[2].to_string());
            } else {
                status = false;
                eprintln!("Incorrect command set");
            }
        } else {
            if self.args.len() == 1 {
                for (var, value) in &shell.variables {
                    let mut aux = String::new();
                    aux.push_str(&var);
                    aux.push_str(" = ");
                    aux.push_str(&value);
                    aux.push('\n');

                    stdout.push_str(&aux);
                }
            } else if self.args.len() == 2 {
                if shell.variables.contains_key(self.args[1]) {
                    stdout.push_str(&shell.variables[self.args[1]]);
                    stdout.push('\n');
                } else {
                    status = false;
                    eprintln!("Variable not found");
                }
            } else {
                status = false;
                eprintln!("Incorrect command get");
            }
        }

        (
            if out {
                print!("{}", stdout);
                -1
            } else {
                str_to_fd(stdout.trim(), shell)
            },
            status,
        )
    }
}

fn fd_to_str(fd: i32) -> String {
    let mut file = unsafe { File::from_raw_fd(fd) };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    buffer
}

fn str_to_fd(buffer: &str, shell: &mut Shell) -> i32 {
    let binding: Vec<&str> = vec![buffer];
    let command = CommandSystem::new("echo", &binding[0..]);
    let (fd, _) = command.execute(shell, -1, false);

    return fd;
}
