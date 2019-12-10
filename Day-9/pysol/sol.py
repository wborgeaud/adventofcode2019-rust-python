from threading import Thread
from queue import Queue
from computer import Computer

program = list(map(int, open('../input.txt').read().split(',')))

def part_one(program):
    c = Computer(program.copy(), [1])
    return c.run()[0]

def part_two(program):
    c = Computer(program.copy(), [2])
    return c.run()[0]

print(f"Part one: {part_one(program)}")
print(f"Part two: {part_two(program)}")


