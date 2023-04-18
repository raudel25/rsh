use format_line::format_line;
use parser::parser;
use std::collections::HashMap;

mod commands;
mod format_line;
mod parser;

pub struct Shell {
    current_command: i32,
    variables: HashMap<String, String>,
    history: Vec<String>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            current_command: -1,
            variables: HashMap::new(),
            history: Vec::new(),
        }
    }

    pub fn execute(&mut self,line: String) {
        let line = format_line(line);
        let args: Vec<&str> = line.trim().split_whitespace().collect();
    
        parser(&args).execute(self,-1, true);
    }
}
