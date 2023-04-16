use commands::{Cd, CommandSystem, Execute};

mod commands;

pub fn execute(line: &str) {
    let args: Vec<&str> = line.trim().split_whitespace().collect();

    parser(&args).execute();
}

fn parser<'a>(args: &'a [&str]) -> Box<dyn Execute + 'a> {
    match args[0] {
        "cd" => Box::new(Cd::new(args)),
        _ => Box::new(CommandSystem::new(args[0], &args[1..])),
    }
}
