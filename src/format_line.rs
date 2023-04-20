pub fn format_line(line: String) -> String {
    let pat = [("&", true), ("|", true), (";", false)];

    let mut new_line = stop_line(line);

    for i in pat {
        new_line = format_pat(&new_line, i.0, i.1);
    }

    new_line = encode_command(new_line);

    return new_line;
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

pub fn decode_command(line: String) -> String {
    let mut new_line1 = String::new();

    let args: Vec<&str> = line.split("( ").collect();

    for i in 0..args.len() - 1 {
        new_line1.push_str(args[i]);
        new_line1.push('`');
    }

    new_line1.push_str(args[args.len() - 1]);

    let mut new_line2 = String::new();

    let args: Vec<&str> = new_line1.split(" )").collect();

    for i in 0..args.len() - 1 {
        new_line2.push_str(args[i]);
        new_line2.push('`');
    }

    new_line2.push_str(args[args.len() - 1]);

    new_line2
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
