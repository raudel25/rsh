use format_line::format_line;
use nix::sys::wait::WaitPidFlag;
use parser::parser;
use std::collections::HashMap;

extern crate colored;
use colored::*;

extern crate rustyline;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::history::{FileHistory, History, SearchDirection};
use rustyline::validate::Validator;
use rustyline::config::Configurer;
use rustyline::{CompletionType, Config, EditMode, Editor, Helper};

use nix::sys::wait::WaitStatus;
use nix::{sys::wait::waitpid, unistd::Pid};

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
    background: Vec<Pid>,
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
        readline.set_max_history_size(1000).unwrap();

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
        if line == "" {
            return;
        }

        let save = line.chars().next().unwrap() != ' ';

        let line = self.again_command(line);
        if save {
            self.readline.add_history_entry(line.as_str()).unwrap();
        }

        let args = format_line(line);
        let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();        

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

    fn special_char(c: char) -> bool {
        return c == '|' || c == ';' || c == '&' || c == '#' || c == '`';
    }

    fn get_again(&self, line: &mut String, index: usize) {
        if index > 0 && index <= self.readline.history().len() {
            let command = self
                .readline
                .history()
                .get(index - 1, SearchDirection::Forward)
                .unwrap()
                .unwrap();
            line.push_str(&command.entry);
        } else {
            eprintln!("{} incorrect command again", error());
            line.push_str("false");
        }
    }

    fn equal_pat(line: &[char], pat: &[char], pos: usize) -> bool {
        if line.len() - pos < pat.len() {
            return false;
        }

        let mut equal = true;
        for j in 0..pat.len() {
            if line[pos + j] != pat[j] {
                equal = false
            };
        }

        equal
    }

    fn again_command(&self, line: String) -> String {
        let mut new_line = String::new();
        let line: Vec<char> = line.chars().collect();

        let pat = ['a', 'g', 'a', 'i', 'n'];
        let pat_help = ['h', 'e', 'l', 'p'];
        let mut next = 0;
        let mut c = false;

        for i in 0..line.len() {
            if '"' == line[i] {
                c = !c;
            }

            if next != 0 {
                next -= 1;
                continue;
            }

            let equal = Shell::equal_pat(&line, &pat, i);
            let equal = equal && !c;

            if equal && i > 0 {
                let mut w = i - 1;

                while w >= pat_help.len() - 1 {
                    w -= 1;
                    if line[w] != ' ' {
                        break;
                    }
                }

                let help = w + 1 >= pat_help.len()
                    && Shell::equal_pat(&line, &pat_help, w + 1 - pat_help.len());
                if help {
                    new_line.push(line[i]);
                    continue;
                }
            }

            if equal {
                if i + pat.len() == line.len() || Shell::special_char(line[i + pat.len()]) {
                    self.get_again(&mut new_line, self.readline.history().len());
                    next = pat.len() - 1;
                    continue;
                }
                let mut s1 = i + pat.len();
                while line[s1] == ' ' {
                    s1 += 1;
                    if s1 == line.len() {
                        break;
                    }
                }

                if s1 == line.len() {
                    self.get_again(&mut new_line, self.readline.history().len());
                    break;
                }

                let mut s2 = s1;
                while line[s2] != ' ' && !Shell::special_char(line[s2]) {
                    s2 += 1;
                    if s2 == line.len() {
                        break;
                    }
                }

                let num = String::from_iter(line[s1..s2].iter());
                let q = num.as_str().parse::<usize>();

                match q {
                    Ok(q) => {
                        self.get_again(&mut new_line, q);
                        next = s2 - i - 1;
                    }
                    Err(_) => {
                        self.get_again(&mut new_line, self.readline.history().len());
                        next = pat.len() - 1;
                    }
                }
            } else {
                new_line.push(line[i]);
            }
        }

        new_line.trim().to_string()
    }

    pub fn update_background(&mut self) {
        let mut s = true;

        while s {
            s = false;
            for i in 0..self.background.len() {
                let wait_status = waitpid(self.background[i], Some(WaitPidFlag::WNOHANG)).unwrap();

                match wait_status {
                    WaitStatus::Exited(_, _) => {
                        println!("[{}]\tDone\t{}", i + 1, self.background[i]);
                        self.background.remove(i);
                        s = true;
                        break;
                    }
                    _ => {}
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
        &self,
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
