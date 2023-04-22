use format_line::{decode_command, format_line};
use parser::parser;
use std::collections::HashMap;

extern crate colored;
use colored::*;

extern crate rustyline;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::history::{FileHistory, History, SearchDirection};
use rustyline::{Editor, Helper,CompletionType, Config, EditMode};
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::Validator;


extern crate libc;
use libc::{c_int, waitpid, WNOHANG};

mod commands;
mod format_line;
mod help;
mod parser;

pub static mut CURRENT_COMMAND: i32 = -1;
pub static mut SIGNAL_CTRL_C: Signal = Signal::Default;

const HISTORY_FILE: &str = ".rsh_history";

pub enum Signal {
    SigInt,
    SigKill,
    Default,
}

fn error() -> ColoredString {
    "rsh:".red()
}
pub struct Shell {
    variables: HashMap<String, String>,
    background: Vec<i32>,
    pub readline: Editor<MyHelper, FileHistory>,
}

impl Shell {
    pub fn new() -> Shell {
        let config = Config::builder()
            .edit_mode(EditMode::Emacs)
            .completion_type(CompletionType::Circular)
            .build();

        let my_helper = MyHelper::new();

        let mut readline = Editor::<MyHelper, FileHistory>::with_config(config).unwrap();
        readline.set_helper(Some(my_helper));

        match readline.load_history(Shell::history_path().as_str()) {
            Ok(_) => {}
            Err(_) => {}
        }

        Shell {
            variables: HashMap::new(),
            background: Vec::new(),
            readline,
        }
    }

    pub fn execute(&mut self, line: String) {
        let line = format_line(line);
        let line = self.again_command(line);

        let save = decode_command(line.clone());
        self.readline.add_history_entry(save.as_str()).unwrap();

        let args: Vec<&str> = line.split_whitespace().collect();

        parser(&args).execute(self, -1, true);
    }

    pub fn prompt() -> String {
        let mut prompt = String::from("rsh@".yellow().to_string().as_str());
        prompt.push_str(Shell::user().yellow().to_string().as_str());
        prompt.push_str(":".yellow().to_string().as_str());
        prompt.push_str(Shell::current_dir().cyan().to_string().as_str());
        prompt.push_str("$ ".cyan().to_string().as_str());

        prompt
    }

    pub fn home() -> String {
        let home = std::env::var("HOME");

        match home {
            Ok(home) => home,
            Err(_) => std::env::var("USERPROFILE").unwrap(),
        }
    }

    pub fn current_dir() -> String {
        std::env::current_dir().unwrap().display().to_string()
    }

    pub fn user() -> String {
        let username = std::env::var("USER");

        match username {
            Ok(username) => username,
            Err(_) => std::env::var("USERNAME").unwrap(),
        }
    }

    fn history_path() -> String {
        let mut path = String::from(Shell::home());
        path.push('/');
        path.push_str(HISTORY_FILE);

        path
    }

    fn again_command(&self, line: String) -> String {
        let mut new_line = String::new();
        let args = line.split_whitespace();

        let mut b = false;

        for i in args {
            if b {
                let number = i.parse::<usize>();

                match number {
                    Ok(number) => {
                        if number > 0 && number <= self.readline.history().len() {
                            let command = self
                                .readline
                                .history()
                                .get(number - 1, SearchDirection::Forward)
                                .unwrap()
                                .unwrap();
                            new_line.push_str(&command.entry);
                        } else {
                            eprintln!("{}: incorrect command again", error());
                            new_line.push_str("false");
                        }

                        new_line.push(' ');
                        b = false;
                        continue;
                    }
                    Err(_) => {
                        let command = self
                            .readline
                            .history()
                            .get(self.readline.history().len() - 1, SearchDirection::Forward)
                            .unwrap()
                            .unwrap();
                        new_line.push_str(&command.entry);
                        new_line.push(' ');
                    }
                }
            }

            b = i == "again";

            if !b {
                new_line.push_str(i);
                new_line.push(' ');
            }
        }

        if b {
            let command = self
                .readline
                .history()
                .get(self.readline.history().len() - 1, SearchDirection::Forward)
                .unwrap()
                .unwrap();
            new_line.push_str(&command.entry);
            new_line.push(' ');
        }

        new_line.trim().to_string()
    }

    pub fn update_background(&mut self) {
        for i in 0..self.background.len() {
            unsafe {
                let mut status: c_int = 0;
                waitpid(self.background[i], &mut status as *mut c_int, WNOHANG);

                if status != 0 {
                    println!("[{}]\tDone\t{}", i + 1, self.background[i]);
                    self.background.remove(i);
                }
            }
        }
    }
}

pub struct MyHelper {
    completer: FilenameCompleter,
    hinter: HistoryHinter,
}

impl MyHelper {
    pub fn new() -> MyHelper {
        let completer = FilenameCompleter::new();
        let hinter = HistoryHinter {};

        MyHelper { completer, hinter }
    }
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Validator for MyHelper {}

impl Hinter for MyHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for MyHelper {}

impl Helper for MyHelper {}
