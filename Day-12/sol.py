import numpy as np
import re

inp = open('input.txt').read().strip().split('\n')

def parse(inp):
    reg = re.compile(r'<x=(.*), y=(.*), z=(.*)>')
    arr = [list(map(int,reg.findall(inp[i])[0])) for i in range(len(inp))]
    return np.array(arr)

def gravity(pos):
    return np.sign(pos[:,np.newaxis]-pos).sum(0)

def energy(pos, v):
    return (np.abs(pos).sum(1)*np.abs(v).sum(1)).sum()

def step(pos,v):
    v += gravity(pos)
    pos += v

def part_one(init_pos):
    pos = init_pos.copy()
    v = np.zeros(pos.shape, dtype=np.int64)
    for _ in range(1000):
        step(pos, v)
    return energy(pos, v)

def cycles(init_pos):
    res = []
    for i in range(3):
        pos = init_pos.copy()
        v = np.zeros(pos.shape, dtype=np.int64)
        step(pos, v)
        ans = 1
        while not np.array_equal(v[:,i], [0,0,0,0]):
            step(pos, v)
            ans += 1
        res.append(2*ans)
    return res
    
def part_two(init_pos):
    lams = cycles(init_pos)
    return np.lcm.reduce(lams)
    

pos = parse(inp)
print(f"Part one: {part_one(pos)}")
print(f"Part two: {part_two(pos)}")

