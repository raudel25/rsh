use format_line::{decode_command, format_line};
use parser::parser;
use std::collections::HashMap;

extern crate colored;
use colored::*;

extern crate rustyline;
use rustyline::history::{History, SearchDirection};
use rustyline::DefaultEditor;

extern crate libc;
use libc::{c_int, waitpid, WNOHANG};

mod commands;
mod format_line;
mod parser;

pub static mut CURRENT_COMMAND: i32 = -1;
pub static mut SIGNAL_CTRL_C: bool = true;

const HISTORY_FILE: &str = ".rsh_history";

fn error() -> ColoredString {
    "rsh:".red()
}
pub struct Shell {
    variables: HashMap<String, String>,
    background: Vec<i32>,
    pub readline: DefaultEditor,
}

impl Shell {
    pub fn new() -> Shell {
        let mut readline = DefaultEditor::new().unwrap();

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
        for i in 0..self.background.len(){
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
