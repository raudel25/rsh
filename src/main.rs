use std::io::{self, Write};

extern crate libc;
use libc::{c_int, kill, signal, SIGINT, SIGKILL};

use rsh::{Shell, Signal, CURRENT_COMMAND, SIGNAL_CTRL_C};

extern "C" fn ctrl_c(_: c_int) {
    unsafe {
        if CURRENT_COMMAND == -1 {
            return;
        }

        SIGNAL_CTRL_C = match SIGNAL_CTRL_C {
            Signal::Default => Signal::SigInt,
            Signal::SigInt => Signal::SigKill,
            Signal::SigKill => Signal::SigKill,
        };

        match SIGNAL_CTRL_C {
            Signal::SigInt => {
                kill(CURRENT_COMMAND, SIGINT);
            }
            Signal::SigKill => {
                kill(CURRENT_COMMAND, SIGKILL);
            }
            _ => {}
        }
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
            SIGNAL_CTRL_C = Signal::Default;
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
