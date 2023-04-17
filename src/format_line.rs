pub fn format_line(line: String) -> String {
    let pat = [("&", true), ("|", true), (";", false)];

    let mut new_line = stop_line(line);

    for i in pat {
        new_line = format_pat(&new_line, i.0, i.1);
    }

    return new_line;
}

fn stop_line(line: String) -> String {
    let mut args = line.split("#");

    return String::from(args.next().unwrap());
}

fn format_pat(line: &String, pat: &str, par: bool) -> String {
    let mut new_line = String::new();

    let args: Vec<&str> = line.split(pat).collect();

    if args.len() == 1 {
        new_line.push_str(line.trim());
        return new_line;
    }

    new_line.push_str(args[0].trim());
    new_line.push_str(" ");
    new_line.push_str(pat);

    for i in 1..args.len() - 1 {
        if args[i] == "" && par {
            new_line.push_str(pat);
            continue;
        };

        new_line.push_str(" ");
        new_line.push_str(args[i].trim());
        new_line.push_str(" ");
        new_line.push_str(pat);
    }

    new_line.push_str(" ");
    new_line.push_str(args[args.len() - 1].trim());

    return new_line;
}
