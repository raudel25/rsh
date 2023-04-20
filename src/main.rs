use std::io::{self, Write};

extern crate libc;
use libc::{c_int, kill, signal, SIGINT, SIGKILL};

use rsh::{Shell, CURRENT_COMMAND, SIGNAL_CTRL_C};

extern "C" fn ctrl_c(_: c_int) {
    unsafe {
        if CURRENT_COMMAND == -1 {
            return;
        }

        kill(
            CURRENT_COMMAND,
            if SIGNAL_CTRL_C { SIGINT } else { SIGKILL },
        );
    }
}

fn main() {
    let mut rsh = Shell::new();

    unsafe {
        signal(SIGINT, ctrl_c as usize);
    }

    loop {
        io::stdout().flush().unwrap();

        unsafe {
            CURRENT_COMMAND = -1;
            SIGNAL_CTRL_C = true;
        }
        rsh.update_background();

        let line = rsh.readline.readline(Shell::prompt().as_str());

        match line {
            Ok(line) => {
                rsh.execute(line);
            }
            Err(_) => {}
        };
    }
}
