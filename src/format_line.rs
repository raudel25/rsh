use crate::error;

pub fn format_line(line: String) -> Vec<String> {
    let pat = [
        ("&", true),
        ("|", true),
        (";", false),
        ("<", false),
        (">", true),
        ("(", false),
        (")", false),
    ];

    let new_line = stop_line(line);
    let (mut new_line, v) = encode_c(new_line);

    for i in pat {
        new_line = format_pat(&new_line, i.0, i.1);
    }

    new_line = encode_command(new_line);
    new_line = eliminate_spaces(new_line);

    let args: Vec<&str> = new_line.split_whitespace().collect();

    decode_c(args, v)
}

fn stop_line(line: String) -> String {
    let mut args = line.split("#");

    return String::from(args.next().unwrap());
}

fn encode_command(line: String) -> String {
    let mut new_line = String::new();

    let mut o = true;

    for i in line.chars() {
        if i == '`' {
            if o {
                new_line.push_str("( ");
            } else {
                new_line.push_str(" )");
            };
        } else {
            new_line.push(i);
        }

        o = i == ' ';
    }

    return new_line;
}

fn eliminate_spaces(line: String) -> String {
    let mut new_line = String::new();
    let args: Vec<&str> = line.split_whitespace().collect();

    for i in 0..args.len() - 1 {
        if args[i] == "" {
            continue;
        }

        new_line.push_str(format!("{} ", args[i]).as_str());
    }

    new_line.push_str(args[args.len() - 1]);

    new_line
}

fn encode_c(line: String) -> (String, Vec<String>) {
    let mut new_line = String::new();
    let mut v = Vec::new();

    let args: Vec<&str> = line.split("\"").collect();

    if args.len() % 2 == 0 {
        eprintln!("{} incorrect format line", error());

        return (String::from("false"), v);
    }

    for i in 0..args.len() {
        if i % 2 == 0 {
            new_line.push_str(args[i]);
        } else {
            v.push(args[i].to_string());
            new_line.push_str("\"")
        }
    }

    (new_line, v)
}

fn decode_c(args: Vec<&str>, v: Vec<String>) -> Vec<String> {
    let mut new_args: Vec<String> = Vec::new();

    let mut j = 0;
    for arg in args {
        let mut aux = String::new();

        for i in arg.chars() {
            if i == '"' {
                aux.push_str(v[j].as_str());
                j += 1;
            } else {
                aux.push(i);
            }
        }

        new_args.push(aux);
    }

    new_args
}

fn format_pat(line: &String, pat: &str, par: bool) -> String {
    let mut new_line = String::new();

    let args: Vec<&str> = line.split(pat).collect();

    if args.len() == 1 {
        new_line.push_str(line.trim());
        return new_line;
    }

    new_line.push_str(format!("{} {}", args[0].trim(), pat).as_str());

    for i in 1..args.len() - 1 {
        if args[i] == "" && par {
            new_line.push_str(pat);
            continue;
        };

        new_line.push_str(format!(" {} {}", args[i].trim(), pat).as_str());
    }

    new_line.push_str(format!(" {}", args[args.len() - 1].trim()).as_str());

    return new_line;
}
