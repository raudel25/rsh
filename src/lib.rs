use format_line::format_line;
use parser::parser;

mod commands;
mod format_line;
mod parser;

pub fn execute(line: String) {
    let line = format_line(line);
    let args: Vec<&str> = line.trim().split_whitespace().collect();

    parser(&args).execute(-1, true);
}
