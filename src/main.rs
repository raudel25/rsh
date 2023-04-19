use rustyline::DefaultEditor;
use std::io::{self, Write};

use rsh::Shell;

fn main() {
    let mut rsh = Shell::new();
    // let mut rl = DefaultEditor::new().unwrap();

    loop {
        io::stdout().flush().unwrap();

        let line = rsh.readline.readline(">> ").unwrap();

        // rl.add_history_entry(line.as_str()).unwrap();

        rsh.execute(line);
    }
}
