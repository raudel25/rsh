use super::commands::*;

pub fn parser<'a>(args: &'a [&str]) -> Box<dyn Execute + 'a> {
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
        "<" => redirect(args, ind, Redirect::In),
        "|" => pipes(args, ind),
        ">" => redirect(args, ind, Redirect::Out),
        ">>" => redirect(args, ind, Redirect::OutAppend),
        "&&" => chain(args, ind, Chain::And),
        "||" => chain(args, ind, Chain::Or),
        ";" => chain(args, ind, Chain::Multiple),
        "cd" => Box::new(Cd::new(args)),
        "history" => Box::new(HistoryCommand::new()),
        "get" => Box::new(GetSet::new(args, true)),
        "set" => Box::new(GetSet::new(args, false)),
        "true" => Box::new(SpecialCommand::new(Special::True)),
        "false" => Box::new(SpecialCommand::new(Special::False)),
        "exit" => Box::new(SpecialCommand::new(Special::Exit)),
        _ => Box::new(CommandSystem::new(args[0], &args[1..])),
    }
}

fn priority_command(arg: &str) -> u16 {
    match arg {
        "<" => 2,
        "|" => 3,
        ">" => 4,
        ">>" => 4,
        "&&" => 5,
        "||" => 5,
        ";" => 6,
        _ => 0,
    }
}

fn chain<'a>(args: &'a [&str], ind: usize, chain: Chain) -> Box<dyn Execute + 'a> {
    if (ind == 0 || ind == args.len() - 1) && chain != Chain::Multiple {
        eprintln!("Incorrect Chain");
        return Box::new(SpecialCommand::new(Special::False));
    }

    let c1 = if ind == 0 {
        Box::new(SpecialCommand::new(Special::True))
    } else {
        parser(&args[0..ind])
    };
    let c2 = if ind == args.len() - 1 {
        Box::new(SpecialCommand::new(Special::True))
    } else {
        parser(&args[ind + 1..])
    };

    Box::new(ChainCommand::new(c1, c2, chain))
}

fn pipes<'a>(args: &'a [&str], ind: usize) -> Box<dyn Execute + 'a> {
    if ind == 0 || ind == args.len() - 1 {
        eprintln!("Incorrect Pipe");
        return Box::new(SpecialCommand::new(Special::False));
    }

    let c1 = parser(&args[0..ind]);
    let c2 = parser(&args[ind + 1..]);

    Box::new(Pipe::new(c1, c2))
}

fn redirect<'a>(args: &'a [&str], ind: usize, redirect_command: Redirect) -> Box<dyn Execute + 'a> {
    if ind == 0 || ind == args.len() - 1 {
        eprintln!("Incorrect Redirect");
        return Box::new(SpecialCommand::new(Special::False));
    }

    let c = parser(&args[0..ind]);

    Box::new(RedirectCommand::new(c, redirect_command, args[ind + 1]))
}
