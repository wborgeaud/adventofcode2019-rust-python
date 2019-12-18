from threading import Thread
from queue import Queue
from computer import Computer

program = list(map(int, open('../input.txt').read().split(',')))

def get_map(program):
    queue_out = Queue()
    queue_in = Queue()
    c = Computer(program.copy(), queue_in, queue_out)
    Thread(target=c.run).start()
    res = ['']
    while not queue_out.empty() or not c.finished:
        out = queue_out.get()
        if out == 10:
            res += ['']
        else:
            res[-1] += chr(out)
    return [x for x in res if x]

def find_inter(m):
    inter = []
    for i in range(1,len(m)-1):
        for j in range(1,len(m[0])-1):
            if (m[i][j], m[i-1][j],m[i+1][j],m[i][j-1],m[i][j+1])==5*('#',):
                inter.append((i,j))
    return inter

def part_one(program):
    m = get_map(program.copy())
    inter = find_inter(m)
    return sum([x[0]*x[1] for x in inter])

def solve_it(program):
    routines = 'A,B,B,C,C,A,B,B,C,A'
    ra = 'R,4,R,12,R,10,L,12'
    rb = 'L,12,R,4,R,12'
    rc = 'L,12,L,8,R,10'

    queue_out = Queue()
    queue_in = Queue()
    c = Computer(program.copy(), queue_in, queue_out)
    Thread(target=c.run).start()
    for s in [routines, ra, rb, rc]:
        for x in s:
            queue_in.put(ord(x))
        queue_in.put(10)
    queue_in.put(ord('n'))
    queue_in.put(10)
    while not queue_out.empty() or not c.finished:
        out = queue_out.get()
    return out

def part_two(init_program):
    program = init_program.copy()
    program[0] = 2
    return solve_it(program)

print(f"Part one: {part_one(program)}")
print(f"Part two: {part_two(program)}")
