from threading import Thread
from queue import Queue
from computer import Computer

program = list(map(int, open('../input.txt').read().split(',')))

code1 = """NOT C J
AND D J
NOT A T
AND D T
OR T J
WALK
"""
code2 = """NOT C J
AND D J
AND H J
NOT A T
AND D T
OR T J
NOT B T
AND D T
OR T J
RUN
"""

def run_program(code,program):
    inst = list(map(ord, code))
    queue_out = Queue()
    queue_in = Queue()
    c = Computer(program.copy(), queue_in, queue_out)
    Thread(target=c.run).start()
    for x in inst:
        queue_in.put(x)
    while not c.finished:pass
    while not queue_out.empty():
        x = queue_out.get()
    return x

def part_one(program):
    return run_program(code1,program)

def part_two(program):
    return run_program(code2,program)

print(f"Part one: {part_one(program)}")
print(f"Part two: {part_two(program)}")
