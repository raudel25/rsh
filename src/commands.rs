use std::env;
use std::fs::{File, OpenOptions};
use std::io::copy;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(PartialEq)]
pub enum Redirect {
    RedirectIn,
    RedirectOut,
    RedirectOutAppend,
}
pub trait Execute {
    fn execute(&self, stdin: Stdio, out: bool) -> i32;
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
    fn execute(&self, stdin: Stdio, out: bool) -> i32 {
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
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                };

                if !out {
                    return command.stdout.take().unwrap().into_raw_fd();
                }
                return -1;
            }
            Err(e) => eprintln!("{}", e),
        };

        -1
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
    fn execute(&self, _: Stdio, _: bool) -> i32 {
        let new_dir = self.args[1];

        let root = Path::new(new_dir);

        if let Err(e) = env::set_current_dir(&root) {
            eprintln!("{}", e);
        }

        -1
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
    fn execute(&self, stdin: Stdio, out: bool) -> i32 {
        let stdout = self.command1.execute(stdin, false);
        let stdin = if stdout == -1 {
            Stdio::inherit()
        } else {
            unsafe { Stdio::from_raw_fd(stdout) }
        };
        self.command2.execute(stdin, out)
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
    fn execute(&self, stdin: Stdio, out: bool) -> i32 {
        let file = match self.redirect {
            Redirect::RedirectIn => File::open(self.path),
            Redirect::RedirectOut => OpenOptions::new().write(true).create(true).open(self.path),
            Redirect::RedirectOutAppend => {
                OpenOptions::new().write(true).create(true).append(true).open(self.path)
            }
        };

        match file {
            Ok(mut file) => {
                let stdin = if self.redirect == Redirect::RedirectIn {
                    unsafe { Stdio::from_raw_fd(file.as_raw_fd()) }
                } else {
                    stdin
                };

                let out = if self.redirect == Redirect::RedirectIn {
                    out
                } else {
                    false
                };

                let stdout = self.command.execute(stdin, out);

                if self.redirect == Redirect::RedirectIn || stdout == -1 {
                    return stdout;
                }

                let mut out_file = unsafe { File::from_raw_fd(stdout) };

                match copy(&mut out_file, &mut file) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                };

                -1
            }
            Err(e) => {
                eprintln!("{}", e);
                -1
            }
        }
    }
}

pub struct False {}

impl Execute for False {
    fn execute(&self, _: Stdio, _: bool) -> i32 {
        -1
    }
}
