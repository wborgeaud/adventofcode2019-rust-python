inp = open('input.txt').read().strip()
inp = """R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"""

def parse_input(inp):
    pinp = inp.split('\n')
    pinp = list(map(lambda x:x.split(','), pinp))
    pinp[0] = [(x[0],int(x[1:])) for x in pinp[0]]
    pinp[1] = [(x[0],int(x[1:])) for x in pinp[1]]
    return pinp

def fill_points(instructions):
    point = (0,0)
    points = set()
    for x in instructions:
        if x[0] == 'R':
            for i in range(1,x[1]+1):
                npoint = (point[0]+i, point[1])
                points.add(npoint)
        elif x[0] == 'L':
            for i in range(1,x[1]+1):
                npoint = (point[0]-i, point[1])
                points.add(npoint)
        elif x[0] == 'U':
            for i in range(1,x[1]+1):
                npoint = (point[0], point[1]+i)
                points.add(npoint)
        elif x[0] == 'D':
            for i in range(1,x[1]+1):
                npoint = (point[0], point[1]-i)
                points.add(npoint)
        else:
            raise Exception("Parsing Error")
        point = npoint
    return points

def part_one(inp):
    points1 = fill_points(inp[0])
    points2 = fill_points(inp[1])
    intersection = [x for x in points2 if x in points1]
    return min([abs(x)+abs(y) for x,y in intersection])

inp = parse_input(inp)
print(f"Part one: {part_one(inp)}")