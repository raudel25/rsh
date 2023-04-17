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
    fn execute(&self, stdin: i32, out: bool) -> (i32, bool);
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
    fn execute(&self, stdin: i32, out: bool) -> (i32, bool) {
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
    fn execute(&self, _: i32, _: bool) -> (i32, bool) {
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
    fn execute(&self, stdin: i32, out: bool) -> (i32, bool) {
        let (stdout, status1) = self.command1.execute(stdin, false);
        let (stdout, status2) = self.command2.execute(stdout, out);

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
    fn execute(&self, stdin: i32, out: bool) -> (i32, bool) {
        let file = match self.redirect {
            Redirect::RedirectIn => File::open(self.path),
            Redirect::RedirectOut => OpenOptions::new().write(true).create(true).open(self.path),
            Redirect::RedirectOutAppend => OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(self.path),
        };

        match file {
            Ok(mut file) => {
                let stdin = if self.redirect == Redirect::RedirectIn {
                    file.as_raw_fd()
                } else {
                    stdin
                };

                let out = if self.redirect == Redirect::RedirectIn {
                    out
                } else {
                    false
                };

                let (stdout, status) = self.command.execute(stdin, out);

                if self.redirect == Redirect::RedirectIn || stdout == -1 {
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

pub struct AndOr<'a> {
    command1: Box<dyn Execute + 'a>,
    command2: Box<dyn Execute + 'a>,
    and: bool,
}

impl AndOr<'_> {
    pub fn new<'a>(
        command1: Box<dyn Execute + 'a>,
        command2: Box<dyn Execute + 'a>,
        and: bool,
    ) -> AndOr<'a> {
        AndOr {
            command1,
            command2,
            and,
        }
    }
}

impl Execute for AndOr<'_> {
    fn execute(&self, stdin: i32, out: bool) -> (i32, bool) {
        let (stdout, status) = self.command1.execute(stdin, out);

        if (self.and && status) || (!self.and && !status) {
            return self.command2.execute(-1, true);
        }

        (stdout, status)
    }
}

pub struct False {}

impl Execute for False {
    fn execute(&self, _: i32, _: bool) -> (i32, bool) {
        (-1, false)
    }
}
