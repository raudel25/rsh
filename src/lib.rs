use format_line::{decode_command, format_line};
use parser::parser;
use rustyline::history::{History, SearchDirection};
use rustyline::DefaultEditor;
use std::collections::HashMap;

mod commands;
mod format_line;
mod parser;

const HISTORY_FILE: &str = ".rsh_history";

pub struct Shell {
    current_command: i32,
    variables: HashMap<String, String>,
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
            current_command: -1,
            variables: HashMap::new(),
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

    pub fn home() -> String {
        std::env::home_dir().unwrap().display().to_string()
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
                            eprintln!("Incorrect command again");
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
}
