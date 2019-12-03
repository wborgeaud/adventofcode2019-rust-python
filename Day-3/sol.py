inp = open('input.txt').read().strip()

def parse_input(inp):
    pinp = inp.split('\n')
    pinp = list(map(lambda x:x.split(','), pinp))
    pinp[0] = [(x[0],int(x[1:])) for x in pinp[0]]
    pinp[1] = [(x[0],int(x[1:])) for x in pinp[1]]
    return pinp

directions = {'R':(1,0), 'L':(-1,0), 'U':(0,1), 'D':(0,-1)}

def fill_points(instructions):
    point = (0,0)
    points = {}
    step = 1
    for x in instructions:
        direction = directions[x[0]]
        for i in range(1,x[1]+1):
            npoint = (point[0]+i*direction[0], point[1]+i*direction[1])
            if npoint not in points:
                points[npoint] = step
            step += 1
        point = npoint
    return points

def part_one(intersection):
    return min([abs(x)+abs(y) for x,y in intersection])

def part_two(intersection, points1, points2):
    return min([points1[x]+points2[x] for x in intersection])

inp = parse_input(inp)
points1 = fill_points(inp[0])
points2 = fill_points(inp[1])
intersection = [x for x in points2 if x in points1]
print(f"Part one: {part_one(intersection)}")
print(f"Part one: {part_two(intersection, points1, points2)}")
