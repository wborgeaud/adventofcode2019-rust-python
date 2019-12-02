program = list(map(int,open('input.txt').read().split(',')))

class Segfault(Exception):
    pass

def opcode(ip, program):
    a, b, c = program[ip+1:ip+4]
    if max(a, b, c) >= len(program):
        raise Segfault
    if program[ip] == 1:
        program[c] = program[a] + program[b]
    elif program[ip] == 2:
        program[c] = program[a] * program[b]
    return ip + 4

def run(program):
    ip = 0
    while program[ip] != 99:
        ip = opcode(ip, program)
    return program

def prepare(program, a, b):
    program[1] = a
    program[2] = b

def part_one(program):
    prog_copy = program.copy()
    prepare(prog_copy, 12, 2)
    return run(prog_copy)[0]

def part_two(program):
    for a in range(100):
        for b in range(100):
            prog_copy = program.copy()
            prepare(prog_copy, a, b)
            try:
                if run(prog_copy)[0] == 19690720:
                    return a*100 + b
            except Segfault:
                pass

print(f"Part one: {part_one(program)}")
print(f"Part two: {part_two(program)}")
