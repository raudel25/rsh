use crate::error;

pub fn format_line(line: String) -> Vec<String> {
    let (new_line, special_spaces) = format_encode(line);

    let args: Vec<&str> = new_line.split_whitespace().collect();

    decode(args, special_spaces)
}

fn decode(args: Vec<&str>, special_spaces: Vec<usize>) -> Vec<String> {
    let mut new_args: Vec<String> = Vec::new();
    let mut x = 0;
    let mut j = 0;

    for arg in args {
        let mut aux = String::new();

        for i in arg.chars() {
            if special_spaces.len() != x && j == special_spaces[x] {
                aux.push(' ');
                x += 1;
            } else {
                aux.push(i);
            }
            j += 1;
        }
        j += 1;

        new_args.push(aux);
    }

    new_args
}

fn format_encode(line: String) -> (String, Vec<usize>) {
    let line: Vec<char> = line.chars().collect();
    let mut new_line = String::new();
    let mut special_spaces: Vec<usize> = Vec::new();

    let mut c = false;
    let mut cont = false;

    for i in 0..line.len() {
        if cont {
            cont = false;
            continue;
        }

        if line[i] == '"' {
            c = !c;
            continue;
        }

        if c {
            if line[i] == ' ' {
                format_special_spaces(&mut new_line, &mut special_spaces);
            } else {
                new_line.push(line[i])
            };
            continue;
        }

        if line[i] == '#' {
            break;
        }

        let mut stop = format_special(&line, i, &mut new_line, &mut special_spaces, &mut cont);
        stop = stop || format_spaces(&line, i);
        stop = stop || format_set(&line, i, &mut new_line);
        stop = stop || format_pat(&line, i, &mut new_line);

        if stop {
            continue;
        }

        new_line.push(line[i]);
    }

    if c {
        eprintln!("{} incorrect format line", error());

        return (String::from("false"), special_spaces);
    }

    (new_line, special_spaces)
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

fn format_spaces(line: &[char], ind: usize) -> bool {
    if line[ind] == ' ' {
        if ind != 0 {
            if line[ind - 1] == ' ' {
                return true;
            }
        } else {
            return true;
        }
    }

    false
}

fn format_set(line: &[char], ind: usize, new_line: &mut String) -> bool {
    if line[ind] == '`' {
        if ind == 0 || line[ind - 1] == ' ' {
            new_line.push_str("( ");
        } else {
            new_line.push_str(" )");
        };

        return true;
    }

    false
}

fn format_special(
    line: &[char],
    ind: usize,
    new_line: &mut String,
    special_spaces: &mut Vec<usize>,
    cont: &mut bool,
) -> bool {
    let special = ['\\', '"', '`', ' ', '#'];

    if line[ind] == '\\' {
        if ind != line.len() - 1 {
            for x in special {
                if x == line[ind + 1] {
                    if x != ' ' {
                        new_line.push(x);
                    } else {
                        format_special_spaces(new_line, special_spaces);
                    }
                    *cont = true;
                    break;
                }
            }
        }
        return true;
    }

    false
}

fn format_special_spaces(new_line: &mut String, special_spaces: &mut Vec<usize>) {
    special_spaces.push(new_line.len());
    new_line.push('#');
}
