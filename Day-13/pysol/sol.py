from threading import Thread
from queue import Queue
from computer import Computer
import numpy as np

program = list(map(int, open('../input.txt').read().split(',')))

def make_tiles(program):
    tiles = {}
    queue_out = Queue()
    c = Computer(program.copy(), Queue(), queue_out)
    Thread(target=c.run).start()
    while not c.finished:
        out = []
        for _ in range(3):
            out.append(queue_out.get())
        tiles[(out[0],out[1])] = out[2]
    return tiles

def play(program):
    game = np.zeros((37,20), dtype=np.int64)
    queue_out = Queue()
    queue_in = Queue()
    c = Computer(program.copy(), queue_in, queue_out)
    Thread(target=c.run).start()
    queue_in.put(0)
    score = 0
    while not c.finished or not queue_out.empty():
        out = []
        for _ in range(3):
            out.append(queue_out.get())
        if out[0]==-1 and out[1]==0:
            score = out[2]
        else:
            game[out[0],out[1]] = out[2]
        if out[2] == 4 and 3 in game:
            paddle_pos = np.where(game==3)
            paddle_pos = (paddle_pos[0][0], paddle_pos[1][0])
            if out[0]>paddle_pos[0]:
                queue_in.put(1)
            elif out[0]<paddle_pos[0]:
                queue_in.put(-1)
            else:
                queue_in.put(0)
    return score

def part_one(program):
    tiles = make_tiles(program.copy())
    return len([x for x in tiles if tiles[x]==2])

def part_two(init_program):
    program = init_program.copy()
    program[0] = 2
    return play(program)

print(f"Part one: {part_one(program)}")
print(f"Part two: {part_two(program)}")
