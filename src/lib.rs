use commands::{Cd, CommandSystem, Execute, False, Pipe};
use std::process::Stdio;

mod commands;

pub fn execute(line: &str) {
    let args: Vec<&str> = line.trim().split_whitespace().collect();

    parser(&args).execute(Stdio::inherit(), true);
}

fn priority_command(arg: &str) -> u16 {
    match arg {
        "|" => 2,
        ">" => 2,
        ">>" => 2,
        "<" => 2,
        _ => 0,
    }
}

fn parser<'a>(args: &'a [&str]) -> Box<dyn Execute + 'a> {
    let mut ind = 0;
    let mut priority = 1;

    for i in 0..args.len() {
        let aux = priority_command(args[i]);

        if aux > priority {
            priority = aux;
            ind = i;
        }
    }

    match args[ind] {
        "|" => {
            let c1 = if ind != 0 {
                parser(&args[0..ind])
            } else {
                eprintln!("Incorrect Pipe");
                Box::new(False {})
            };
            let c2 = if ind != args.len() - 1 {
                parser(&args[ind + 1..])
            } else {
                eprintln!("Incorrect Pipe");
                Box::new(False {})
            };

            Box::new(Pipe::new(c1, c2))
        }
        "cd" => Box::new(Cd::new(args)),
        _ => Box::new(CommandSystem::new(args[0], &args[1..])),
    }
}
