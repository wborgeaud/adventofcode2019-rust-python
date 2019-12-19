from threading import Thread
from queue import Queue
from computer import Computer

program = list(map(int, open('../input.txt').read().split(',')))

def get_coords(x, y, program):
    queue_out = Queue()
    queue_in = Queue()
    c = Computer(program.copy(), queue_in, queue_out)
    Thread(target=c.run).start()
    queue_in.put(x)
    queue_in.put(y)
    return queue_out.get()

def find_max_y(x, program):
    y, Y = 0, x/0.6
    while True:
        m = (y+Y)//2
        if get_coords(x, m, program):
            Y = m
        else:
            y = m
        if y == Y-1:
            return Y

def is_good(x, y, program):
    return get_coords(x, y, program) and \
        get_coords(x, y+99, program) and \
        get_coords(x-99, y+99, program) and \
        get_coords(x-99, y, program)

def part_one(program):
    ans = 0
    for x in range(50):
        for y in range(50):
            ans += get_coords(x, y, program)
    return ans

def part_two(program):
    x, X = 0, 10000
    while True:
        m = (x+X)//2
        y = find_max_y(m, program)
        if is_good(m, y, program):
            X = m
        else:
            x = m
        if x == X-1:
            return (X-99)*10000+find_max_y(X, program)

print(f"Part one: {part_one(program)}")
print(f"Part two: {part_two(program)}")