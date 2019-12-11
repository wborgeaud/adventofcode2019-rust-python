from threading import Thread
from queue import Queue
from computer import Computer

program = list(map(int, open('../input.txt').read().split(',')))

def paint(program, first_color):
    panels = {}
    pos, last_pos = (0,0), (0,-1)
    queue_in = Queue()
    queue_out = Queue()
    c = Computer(program.copy(), queue_in, queue_out)
    Thread(target=c.run).start()
    while not c.finished:
        current_color = panels[pos] if pos in panels else 0 if len(panels)>0 else first_color
        queue_in.put(current_color)
        new_color = queue_out.get()
        direction = queue_out.get()
        panels[pos] = new_color
        if direction:
            pos, last_pos = (pos[0] + pos[1] - last_pos[1], pos[1] - pos[0] +
                             last_pos[0]), pos
        else:
            pos, last_pos = (pos[0] - pos[1] + last_pos[1], pos[1] + pos[0] -
                             last_pos[0]), pos
    return panels

def part_one(program):
    panels = paint(program, 0)
    return len(panels)

def part_two(program):
    panels = paint(program, 1)
    xs = [x[0] for x in panels if panels[x]]
    ys = [x[1] for x in panels if panels[x]]
    mx, Mx = min(xs), max(xs)
    my, My = min(ys), max(ys)
    res = "\n"
    for j in range(My, my-1,-1):
        res += ''.join(['⬛' if panels.get((i,j)) else ' ' for i in range(mx,Mx+1)])
        res += '\n'
    return res

        
print(f"Part one: {part_one(program)}")
print(f"Part two: {part_two(program)}")

