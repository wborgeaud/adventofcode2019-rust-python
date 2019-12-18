from threading import Thread, Event
from queue import Queue
from computer import Computer
import sys

program = list(map(int, open('../input.txt').read().split(',')))
opposite = {
    1: 2,
    2: 1,
    3: 4,
    4: 3
}
coords = {
    1: (0,1),
    2: (0, -1),
    3: (1,0),
    4: (-1, 0)
}

def try_path(path,program):
    queue_out = Queue()
    queue_in = Queue()
    c = Computer(program.copy(), queue_in, queue_out)
    Thread(target=c.run).start()
    for v in path:
        queue_in.put(v)
        out = queue_out.get()
    return out

def get_map(program):
    m = {(0,0):(1,0)}
    queue = [((),(0,0))]
    while len(queue)>0:
        vertex = queue.pop(0)
        for d in opposite:
            if len(vertex[0]) and d == opposite[vertex[0][-1]]:
                continue
            out = try_path(vertex[0] + (d,), program)
            nv = (vertex[1][0]+coords[d][0], vertex[1][1]+coords[d][1])
            m[nv] = (out, len(vertex[0])+1)
            if out == 0:
                continue
            elif out == 1:
                queue.append((vertex[0] + (d,), nv))
            elif out == 2:
                queue.append((vertex[0] + (d,), nv))
    return m

def bfs(start, m):
    queue = [(start,0)]
    seen = set([start])
    maxlen = 0
    while len(queue)>0:
        vertex = queue.pop(0)
        if vertex[1]>maxlen:
            maxlen = vertex[1]
        for d in opposite:
            nv = (vertex[0][0]+coords[d][0], vertex[0][1]+coords[d][1])
            if nv in seen:
                continue
            else:
                seen.add(nv)
            out = m[nv][0]
            if out == 0:
                continue
            elif out == 1:
                queue.append((nv, vertex[1]+1))
            elif out == 2:
                queue.append((nv, vertex[1]+1))
    return maxlen


def part_one(m):
    for x in m.values():
        if x[0]==2:
            return x[1]

def part_two(m):
    for x,v in m.items():
        if v[0]==2:
            start = x
            break
    return bfs(start, m)

m = get_map(program)
print(f"Part one: {part_one(m)}")
print(f"Part two: {part_two(m)}")
sys.exit()