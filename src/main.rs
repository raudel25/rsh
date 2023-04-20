use std::io::{self, Write};

use rsh::Shell;

fn main() {
    let mut rsh = Shell::new();

    loop {
        io::stdout().flush().unwrap();

        let line = rsh.readline.readline(Shell::prompt().as_str()).unwrap();

        rsh.execute(line);
    }
}
