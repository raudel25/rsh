use std::io::{self, Write};
use rustyline::DefaultEditor;

use rsh::Shell;

fn main() {
    let mut rsh = Shell::new();
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        io::stdout().flush().unwrap();

        let line =rl.readline(">> ").unwrap();
        
        rsh.execute(line);
    }
}
