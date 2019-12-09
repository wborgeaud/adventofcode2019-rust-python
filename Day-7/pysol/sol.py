from threading import Thread
from queue import Queue
from itertools import permutations
from computer import Computer
import time

program = list(map(int, open('../input.txt').read().split(',')))

def part_one(program):
    m = 0
    for seq in permutations(range(5)):
        queues_in = [Queue() for i in range(5)]
        queues_out = [Queue() for i in range(5)]
        inp = 0
        for i in range(5):
            c = Computer(program.copy(), queues_in[i], queues_out[i])
            t = Thread(target=c.run)
            t.start()
            queues_in[i].put(seq[i])
            queues_in[i].put(inp)
            inp = queues_out[i].get()
            if inp > m:
                m = inp
                best = seq
    return m

def part_two(program):
    m = 0
    for seq in permutations(range(5,10)):
        queues_in = [Queue() for i in range(5)]
        queues_out = [Queue() for i in range(5)]
        computers = []
        for i in range(5):
            c = Computer(program.copy(), queues_in[i], queues_out[i])
            computers.append(c)
            t = Thread(target=c.run)
            t.start()
            queues_in[i].put(seq[i])
        inp = 0
        i = 0
        while True:
            j = i%5
            outs = []
            queues_in[j].put(inp)
            inp = queues_out[j].get()
            if all([c.finished for c in computers]):
                break
            else:
                i += 1

        if inp > m:
            m = inp
            best = seq
    return m

print(f"Part one: {part_one(program)}")
print(f"Part two: {part_two(program)}")

