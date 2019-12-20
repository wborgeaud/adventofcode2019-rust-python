inp = open('input.txt').read().split('\n')[:-1]

def parse(inp):
    n, m = len(inp), len(inp[0])
    floor, gates = set(), {}
    for i in range(n):
        for j in range(m):
            if inp[i][j] == '.':
                floor.add((i, j))
            elif inp[i][j].isupper():
                if i < n-1 and inp[i+1][j].isupper():
                    t = (i-1, j) if inp[i-1][j] == '.' else (i+2, j)
                    if inp[i][j]+inp[i+1][j] in gates:
                        gates[inp[i][j]+inp[i+1][j]].append(t)
                    else:
                        gates[inp[i][j]+inp[i+1][j]] = [t]
                elif j < m-1 and inp[i][j+1].isupper():
                    t = (i, j-1) if inp[i][j-1] == '.' else (i, j+2)
                    if inp[i][j]+inp[i][j+1] in gates:
                        gates[inp[i][j]+inp[i][j+1]].append(t)
                    else:
                        gates[inp[i][j]+inp[i][j+1]] = [t]
    return floor, gates

def neighs(p):
    return [(p[0]+1, p[1]), (p[0], p[1]-1), (p[0]-1, p[1]), (p[0], p[1]+1)]

def make_graph(floor, gates):
    graph = {}
    for n in floor:
        neigh = []
        for x in neighs(n):
            if x in floor:
                neigh.append(x)
        for g in gates.values():
            if len(g) == 2 and n in g:
                neigh.append(g[(g.index(n)+1) % 2])
        graph[n] = neigh
    return graph

def bfs(graph, start, end):
    queue = [(start, 0)]
    seen = set([start])
    while queue:
        v = queue.pop(0)
        if v[0] == end:
            return v[1]
        for n in graph[v[0]]:
            if n not in seen:
                queue.append((n, v[1]+1))
                seen.add(n)

def part_one(inp):
    floor, gates = parse(inp)
    graph = make_graph(floor, gates)
    return bfs(graph, gates['AA'][0], gates['ZZ'][0])

def make_graph2(floor, gates, inp):
    h, w = len(inp), len(inp[0])
    graph = {}
    for n in floor:
        neigh = []
        for x in neighs(n):
            if x in floor:
                neigh.append((x, 0))
        for g in gates.values():
            if len(g) == 2 and n in g:
                if n[0] in [2, h-3] or n[1] in [2, w-3]:
                    neigh.append((g[(g.index(n)+1) % 2], -1))
                else:
                    neigh.append((g[(g.index(n)+1) % 2], 1))
        graph[n] = neigh
    return graph

def bfs2(graph, start, end):
    queue = [(start, 0, 0)]
    seen = set([(start, 0)])
    while queue:
        v = queue.pop(0)
        if v[0] == end and v[1] == 0:
            return v[2]
        for n in graph[v[0]]:
            if v[1] + n[1] < 0:
                continue
            if (n[0], v[1]+n[1]) not in seen:
                queue.append((n[0], v[1]+n[1], v[2]+1))
                seen.add((n[0], v[1]+n[1]))

def part_two(inp):
    floor, gates = parse(inp)
    graph = make_graph2(floor, gates, inp)
    return bfs2(graph, gates['AA'][0], gates['ZZ'][0])

print(f"Part one: {part_one(inp)}")
print(f"Part two: {part_two(inp)}")