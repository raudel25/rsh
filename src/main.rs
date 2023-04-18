use std::io::{self, Write};

use rsh::Shell;

fn main() {
    let mut rsh = Shell::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        rsh.execute(line);
    }
}
