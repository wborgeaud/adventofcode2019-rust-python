import random

inp = open('input.txt').read().split('\n')[:-1]

def get_map(inp): 
    m = {}
    letters = {}
    for i in range(len(inp)):
        for j in range(len(inp[0])):
            if inp[i][j] == '@':
                start = (i,j)
                m[(i,j)] = '.'
            elif inp[i][j] == '#':
                continue
            elif inp[i][j].isalpha() and inp[i][j].islower():
                letters[inp[i][j]] = (i,j)
                m[(i,j)] = inp[i][j]
            else:
                m[(i,j)] = inp[i][j]
    return m, start, letters

def neighs(p, h, w):
    return [(p[0]+1, p[1]), (p[0], p[1]-1), (p[0]-1, p[1]), (p[0], p[1]+1)]

def find_path(s, t, m):
    h, w = len(inp), len(inp[0])
    queue = [(s,[],0)]
    seen = set([s])
    while queue:
        v = queue.pop(0)
        if v[0] == t:
            return v
        for n in neighs(v[0], h, w):
            if n in m and n not in seen:
                if m[n].isupper():
                    queue.append((n, v[1]+[m[n]], v[2]+1))
                else:
                    queue.append((n, v[1], v[2]+1))
                seen.add(n)
    return None
            
def solve(letters, paths):
    letters_list = list(letters)
    queue = [(('start',), 0)]
    while len(queue)>0:
        v = queue.pop()
        if len(v[0])==len(letters)+1:
                return v[0]
        while True:
            l = random.choice(letters_list)
            if l in v[0]: continue
            p = paths[(v[0][-1],l)]
            if all([c.lower() in v[0] for c in p[0]]):
                queue.append((v[0]+(l,),v[1]+p[1]))
                break

def cost(path, paths):
    old = 'start'
    ans = 0
    for i in range(1,len(path)):
        p = paths[(old, path[i])]
        if all([c.lower() in path[:i] for c in p[0]]):
            ans += p[1]
        else:
            return 10000
        old = path[i]
    return ans

def better(path, paths):
    best = path
    min_ = cost(path, paths)
    for i in range(1, len(path)):
        for j in range(i+1, len(path)):
            for k in range(j+1, len(path)):
                l = list(path)
                l[i], l[j], l[k] = l[j], l[k], l[i]
                c = cost(l, paths)
                if c<min_:
                    best = l
                    min_ = c
                l = list(path)
                l[i], l[j], l[k] = l[k], l[i], l[j]
                c = cost(l, paths)
                if c<min_:
                    best = l
                    min_ = c
    return best

def part_one(inp):
    m, start, letters = get_map(inp)
    paths = {}
    for l in letters:
        for ll in letters:
            if l==ll:
                continue
            p = find_path(letters[l], letters[ll], m)
            paths[(l,ll)] = (p[1], p[2])
        p = find_path(letters[l], start, m)
        paths[('start', l)] = (p[1], p[2])
    
    min_ = 10000
    for _ in range(10):
        path = solve(letters, paths)
        c = cost(path, paths)
        while True:
            path = better(path, paths)
            nc = cost(path, paths)
            if nc==c:
                break
            else:
                c = nc
        if c < min_:
            min_ = c
    return min_

def solve2(letters, paths, categories):
    letters_list = list(letters)
    queue =[([(f'start{i}',) for i in range(4)], 0, [])]
    while len(queue)>0:
        v = queue.pop()
        if sum([len(x) for x in v[0]])==len(letters)+4:
            return v[2]
        while True:
            l = random.choice(letters_list)
            i = categories[l]
            if l in v[0][i]: continue
            p = paths[(v[0][i][-1],l)]
            if all([c.lower() in v[0][0]+v[0][1]+v[0][2]+v[0][3] for c in p[0]]):
                nv1 = v[0].copy()
                nv1[i] += (l,)
                queue.append((nv1,v[1]+p[1], v[2]+[(i,l)]))
                break

def cost2(path, paths):
    olds = [f'start{i}' for i in range(4)]
    ans = 0
    for it in range(0,len(path)):
        i,l = path[it]
        p = paths[(olds[i], l)]
        if all([c.lower() in [x[1] for x in path[:it]] for c in p[0]]):
            ans += p[1]
        else:
            return 10000
        olds[i] = l
    return ans

def better2(path, paths):
    best = path
    min_ = cost2(path, paths)
    for i in range(1, len(path)):
        for j in range(i+1, len(path)):
            for k in range(j+1, len(path)):
                l = list(path)
                l[i], l[j], l[k] = l[j], l[k], l[i]
                c = cost2(l, paths)
                if c<min_:
                    best = l
                    min_ = c
                l = list(path)
                l[i], l[j], l[k] = l[k], l[i], l[j]
                c = cost2(l, paths)
                if c<min_:
                    best = l
                    min_ = c
    return best, min_

def part_two(inp):
    m, start, letters = get_map(inp)
    starts = [(start[0]+1, start[1]+1), (start[0]-1, start[1]+1), (start[0]+1, start[1]-1), (start[0]-1, start[1]-1)]
    m.pop(start)
    for x in neighs(start, len(inp), len(inp[0])):
        m.pop(x)
    
    paths = {}
    for l in letters:
        for ll in letters:
            if l==ll:
                continue
            p = find_path(letters[l], letters[ll], m)
            if p:
                paths[(l,ll)] = (p[1], p[2])
        for i in range(4):
            p = find_path(letters[l], starts[i], m)
            if p:
                paths[(f'start{i}', l)] = (p[1], p[2])
    categories = {}
    for l in letters:
        j = [i for i in range(4) if (f'start{i}',l) in paths][0]
        categories[l] = j
    
    min_ = 10000
    for _ in range(10):
        path = solve2(letters, paths, categories)
        c = cost2(path, paths)
        while True:
            path, nc = better2(path, paths)
            if nc==c:
                break
            else:
                c = nc
        if c<min_:
            min_ = c
    return min_

print(f"Part one: {part_one(inp)}")
print(f"Part two: {part_two(inp)}")
