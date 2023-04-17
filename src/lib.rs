use commands::*;

mod commands;

pub fn execute(line: &str) {
    let args: Vec<&str> = line.trim().split_whitespace().collect();

    parser(&args).execute(-1, true);
}

fn priority_command(arg: &str) -> u16 {
    match arg {
        "<" => 2,
        "|" => 3,
        ">" => 4,
        ">>" => 4,
        "&&" => 5,
        "||" => 5,
        _ => 0,
    }
}

fn and_or<'a>(args: &'a [&str], ind: usize, and: bool) -> Box<dyn Execute + 'a> {
    if ind == 0 || ind == args.len() - 1 {
        eprintln!("Incorrect Chain");
        return Box::new(SpecialCommand::new(Special::SpecialFalse));
    }

    let c1 = parser(&args[0..ind]);
    let c2 = parser(&args[ind + 1..]);

    Box::new(AndOr::new(c1, c2, and))
}

fn pipes<'a>(args: &'a [&str], ind: usize) -> Box<dyn Execute + 'a> {
    if ind == 0 || ind == args.len() - 1 {
        eprintln!("Incorrect Pipe");
        return Box::new(SpecialCommand::new(Special::SpecialFalse));
    }

    let c1 = parser(&args[0..ind]);
    let c2 = parser(&args[ind + 1..]);

    Box::new(Pipe::new(c1, c2))
}

fn redirect<'a>(args: &'a [&str], ind: usize, redirect_command: Redirect) -> Box<dyn Execute + 'a> {
    if ind == 0 || ind == args.len() - 1 {
        eprintln!("Incorrect Redirect");
        return Box::new(SpecialCommand::new(Special::SpecialFalse));
    }

    let c = parser(&args[0..ind]);

    Box::new(RedirectCommand::new(c, redirect_command, args[ind + 1]))
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
        "<" => redirect(args, ind, Redirect::RedirectIn),
        "|" => pipes(args, ind),
        ">" => redirect(args, ind, Redirect::RedirectOut),
        ">>" => redirect(args, ind, Redirect::RedirectOutAppend),
        "&&" => and_or(args, ind, true),
        "||" => and_or(args, ind, false),
        "cd" => Box::new(Cd::new(args)),
        "true" => Box::new(SpecialCommand::new(Special::SpecialTrue)),
        "false" => Box::new(SpecialCommand::new(Special::SpecialFalse)),
        "exit" => Box::new(SpecialCommand::new(Special::SpecialExit)),
        _ => Box::new(CommandSystem::new(args[0], &args[1..])),
    }
}
