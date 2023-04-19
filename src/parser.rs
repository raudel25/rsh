use super::commands::*;

pub fn parser<'a>(args: &'a [&str]) -> Box<dyn Execute + 'a> {
    let mut ind = 0;
    let mut priority = 1;
    let mut c_parent = 0;

    for i in 0..args.len() {
        if args[i] == "(" {
            c_parent += 1;
        }
        if args[i] == ")" {
            c_parent -= 1;
        }

        if c_parent != 0 || args[i] == "(" || args[i] == ")" {
            continue;
        }

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
        "set" => set(args),
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
        eprintln!("Incorrect chain");
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
        eprintln!("Incorrect pipe");
        return Box::new(SpecialCommand::new(Special::False));
    }

    let c1 = parser(&args[0..ind]);
    let c2 = parser(&args[ind + 1..]);

    Box::new(Pipe::new(c1, c2))
}

fn redirect<'a>(args: &'a [&str], ind: usize, redirect_command: Redirect) -> Box<dyn Execute + 'a> {
    if ind == 0 || ind == args.len() - 1 {
        eprintln!("Incorrect redirect");
        return Box::new(SpecialCommand::new(Special::False));
    }

    let c = parser(&args[0..ind]);

    Box::new(RedirectCommand::new(c, redirect_command, args[ind + 1]))
}

fn set<'a>(args: &'a [&str]) -> Box<dyn Execute + 'a> {
    if args.len() > 3 {
        if args[2] == "(" && args[args.len() - 1] == ")" {
            let c = parser(&args[3..args.len() - 1]);

            return Box::new(ComplexSet::new(args[1], c));
        } else {
            eprintln!("Incorrect command set");
            return Box::new(SpecialCommand::new(Special::False));
        }
    }

    Box::new(GetSet::new(args, false))
}
