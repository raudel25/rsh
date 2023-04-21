use super::commands::*;
use super::error;

pub fn parser<'a>(args: &'a [&str]) -> Box<dyn Execute + 'a> {
    let mut ind = 0;
    let mut priority = 1;
    let mut c_parent = 0;
    let mut c_cond = 0;

    for i in 0..args.len() {
        if args[i] == "(" {
            c_parent += 1;
        }
        if args[i] == "if" {
            c_cond += 1;
        }
        if args[i] == ")" {
            c_parent -= 1;
        }
        if args[i] == "end" {
            c_cond -= 1;
        }

        if c_parent != 0 || args[i] == "(" || args[i] == ")" {
            continue;
        }
        if c_cond != 0 || args[i] == "if" || args[i] == "end" {
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
        "get" => Box::new(GetSetCommand::new(args, GetSet::Get)),
        "set" => set(args),
        "unset" => Box::new(GetSetCommand::new(args, GetSet::UnSet)),
        "if" => conditional(args),
        "&" => background(args, ind),
        "jobs" => Box::new(Jobs::new()),
        "fg" => Box::new(Foreground::new(args)),
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
        "&" => 7,
        _ => 0,
    }
}

fn chain<'a>(args: &'a [&str], ind: usize, chain: Chain) -> Box<dyn Execute + 'a> {
    if (ind == 0 || ind == args.len() - 1) && chain != Chain::Multiple {
        eprintln!("{} incorrect chain", error());
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
        eprintln!("{} incorrect pipe", error());
        return Box::new(SpecialCommand::new(Special::False));
    }

    let c1 = parser(&args[0..ind]);
    let c2 = parser(&args[ind + 1..]);

    Box::new(Pipe::new(c1, c2))
}

fn redirect<'a>(args: &'a [&str], ind: usize, redirect_command: Redirect) -> Box<dyn Execute + 'a> {
    if ind == 0 || ind == args.len() - 1 {
        eprintln!("{} incorrect redirect", error());
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
            eprintln!("{} incorrect command set", error());
            return Box::new(SpecialCommand::new(Special::False));
        }
    }

    Box::new(GetSetCommand::new(args, GetSet::Set))
}

fn conditional<'a>(args: &'a [&str]) -> Box<dyn Execute + 'a> {
    let mut c_cond = 0;
    let mut pos_then = 0;
    let mut pos_else = 0;
    let mut pos_end = 0;

    for i in 0..args.len() {
        if args[i] == "if" {
            c_cond += 1;
        }
        if args[i] == "end" {
            c_cond -= 1;

            if c_cond == 0 {
                pos_end = i
            }
        }

        if c_cond == 1 {
            match args[i] {
                "then" => pos_then = i,
                "else" => pos_else = i,
                _ => {}
            }
        }
    }

    let mut e = false;

    if c_cond != 0 && pos_then == 0 && pos_end == 0 {
        e = true;
    }

    if pos_else == 0 {
        if pos_then < 2 || pos_end - pos_then < 2 {
            e = true;
        }
    } else {
        if pos_then < 2 || pos_else - pos_then < 2 || pos_end - pos_else < 2 {
            e = true;
        }
    }

    if e {
        eprintln!("{} incorrect conditional", error());

        return Box::new(SpecialCommand::new(Special::False));
    }

    let command = if pos_else == 0 {
        let c1 = parser(&args[1..pos_then]);
        let c2 = parser(&args[pos_then + 1..pos_end]);
        let c3 = Box::new(SpecialCommand::new(Special::False));

        Box::new(Conditional::new(c1, c2, c3))
    } else {
        let c1 = parser(&args[1..pos_then]);
        let c2 = parser(&args[pos_then + 1..pos_else]);
        let c3 = parser(&args[pos_else + 1..pos_end]);

        Box::new(Conditional::new(c1, c2, c3))
    };

    if pos_end == args.len() - 1 {
        command
    } else {
        let c = parser(&args[pos_end + 1..]);

        Box::new(ChainCommand::new(command, c, Chain::Multiple))
    }
}

fn background<'a>(args: &'a [&str], ind: usize) -> Box<dyn Execute + 'a> {
    if ind == 0 {
        return Box::new(SpecialCommand::new(Special::False));
    }

    let command = parser(&args[0..ind]);
    let command = Box::new(Background::new(command));

    if args.len() - 1 == ind {
        command
    } else {
        let c = parser(&args[ind + 1..]);

        Box::new(ChainCommand::new(command, c, Chain::Multiple))
    }
}
