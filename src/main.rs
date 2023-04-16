use std::io::{self, Write};

use rsh::execute;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line == "exit\n" {
            break;
        }

        execute(&line);
    }
}
