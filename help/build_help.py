import os


def parser(s: str) -> str:
    resp = ''
    act = False

    for i in s:
        if i == '\\':
            act = True
            continue

        if i == '\n':
            if act:
                resp += '\\n'
                act = False
            else:
                resp += ' '
            continue

        resp += i if i != '\"' else '\\\"'

    return resp


def to_char_array_c(name: str, files: str) -> str:
    s = 'pub const '+name.upper()+f':[&str;{len(files)}]'+' = [ '
    for i in files:
        s += '"'
        s += i
        s += '",\n'

    s += '];\n'

    return s


files = []
name = []
content = []

for i in os.listdir('.'):
    if i.endswith('.txt'):
        files.append(i)
        name.append(i[3:-4])

for i in files:
    f = open(i, "r")
    aux = parser(f.read())
    content.append(aux)
    f.close()

commands = to_char_array_c('commands', name)
commands_help = to_char_array_c('commands_help', content)

fc = open('../src/help.rs', "w")
fc.write(commands+commands_help)
fc.close()
