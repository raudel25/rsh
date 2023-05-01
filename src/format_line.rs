use crate::error;

pub fn format_line(line: String) -> Vec<String> {
    let new_line = format_encode(line);
    let new_line = encode_command_set(new_line);

    let args: Vec<&str> = new_line.split_whitespace().collect();

    decode(args)
}

fn encode_command_set(line: String) -> String {
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

fn decode(args: Vec<&str>) -> Vec<String> {
    let mut new_args: Vec<String> = Vec::new();

    for arg in args {
        let mut aux = String::new();

        for i in arg.chars() {
            if i == '#' {
                aux.push(' ');
            } else {
                aux.push(i);
            }
        }

        new_args.push(aux);
    }

    new_args
}

fn format_encode(line: String) -> String {
    let line: Vec<char> = line.chars().collect();
    let mut new_line = String::new();

    let mut c = false;
    let mut cont = false;

    for i in 0..line.len() {
        if cont {
            cont = false;
            continue;
        }

        if line[i] == '#' {
            break;
        }

        if line[i] == '"' {
            c = !c;
            continue;
        }

        if c {
            if line[i] == ' ' {
                new_line.push('#');
            } else {
                new_line.push(line[i])
            };
            continue;
        }

        if line[i] == '\\' {
            if i != line.len() - 1 && line[i + 1] == ' ' {
                new_line.push('#');
                cont = true;
            }
            continue;
        }

        if line[i] == ' ' {
            if i != 0 {
                if line[i - 1] == ' ' {
                    continue;
                }
            } else {
                continue;
            }
        }

        let stop = format_pat(&line, i, &mut new_line);

        if stop {
            continue;
        }

        new_line.push(line[i]);
    }

    if c {
        eprintln!("{} incorrect format line", error());

        return String::from("false");
    }

    new_line
}

fn format_pat(line: &[char], ind: usize, new_line: &mut String) -> bool {
    let pat = [
        ('&', true),
        ('|', true),
        (';', false),
        ('<', false),
        ('>', true),
        ('(', false),
        (')', false),
    ];

    for (x, y) in pat {
        if line[ind] == x {
            if ind != 0 {
                if line[ind - 1] != ' ' && line[ind - 1] != x {
                    new_line.push(' ');
                }
            }
            new_line.push(line[ind]);
            if ind != line.len() - 1 {
                if line[ind + 1] != ' ' && (!y || line[ind + 1] != x) {
                    new_line.push(' ');
                }
            }

            return true;
        }
    }

    false
}
