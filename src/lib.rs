use format_line::format_line;
use parser::parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

mod commands;
mod format_line;
mod parser;

const MAX_SIZE_HISTORY: usize = 100;
const HISTORY_FILE: &str = ".rsh_history";

pub struct Shell {
    current_command: i32,
    variables: HashMap<String, String>,
    history: History,
}

impl Shell {
    pub fn new() -> Shell {
        let mut history = History::new();
        Shell::load_history(&mut history);

        Shell {
            current_command: -1,
            variables: HashMap::new(),
            history,
        }
    }

    pub fn execute(&mut self, line: String) {
        let line = format_line(line);
        let args: Vec<&str> = line.trim().split_whitespace().collect();

        parser(&args).execute(self, -1, true);
    }

    pub fn home() -> String {
        std::env::home_dir().unwrap().display().to_string()
    }

    fn load_history(history: &mut History) {
        let mut path = String::from(Shell::home());
        path.push('/');
        path.push_str(HISTORY_FILE);

        let file = File::open(path);

        match file {
            Ok(mut file) => {
                let mut buffer = String::new();

                file.read_to_string(&mut buffer).unwrap();
                let array = buffer.split("\n");

                for command in array {
                    history.push(command.to_string());
                }
            }
            Err(_) => {}
        }
    }
}

struct History {
    init: usize,
    array: Vec<String>,
}

impl History {
    fn new() -> History {
        History {
            init: 0,
            array: Vec::new(),
        }
    }

    fn get(&self, index: usize) -> String {
        if index > self.array.len() {
            panic!("Index out range");
        }

        self.array[(self.init + index) % MAX_SIZE_HISTORY].clone()
    }

    fn push(&mut self, command: String) {
        if self.array.len() == MAX_SIZE_HISTORY {
            self.array[self.init] = command;
            self.init = (self.init + 1) % MAX_SIZE_HISTORY;
        } else {
            self.array.push(command);
        }
    }
}
