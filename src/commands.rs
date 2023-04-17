use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

pub trait Execute {
    fn execute(&self, stdin: Stdio, out: bool) -> Stdio;
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
    fn execute(&self, stdin: Stdio, out: bool) -> Stdio {
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
                    return Stdio::from(command.stdout.unwrap());
                }
                return Stdio::inherit();
            }
            Err(e) => eprintln!("{}", e),
        };

        return Stdio::inherit();
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
    fn execute(&self, _: Stdio, _: bool) -> Stdio {
        let new_dir = self.args[1];

        let root = Path::new(new_dir);

        if let Err(e) = env::set_current_dir(&root) {
            eprintln!("{}", e);
        }

        return Stdio::inherit();
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
    fn execute(&self, stdin: Stdio, out: bool) -> Stdio {
        self.command2
            .execute(self.command1.execute(stdin, false), out)
    }
}

pub struct False {}

impl Execute for False {
    fn execute(&self, _: Stdio, _: bool) -> Stdio {
        return Stdio::inherit();
    }
}
