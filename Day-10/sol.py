from math import inf

inp = open('input.txt').read().strip().split()

def get_map(inp):
    m = {}
    for i in range(len(inp)):
        for j in range(len(inp[0])):
            if inp[i][j]=='#':
                m[(j,i)] = set()
    return m

def are_colinear(a,b):
    if (a[0]*b[0]+a[1]*b[1])<0:
        return False
    return (a[0]*b[1]-a[1]*b[0]) == 0

def sights(m):
    res = {}
    for a in m:
        for k in m:
            if k == a:
                continue
            if not any([are_colinear((s[0]-a[0],s[1]-a[1]),
                                     (k[0]-a[0],k[1]-a[1])) for s in m[a]]):
                m[a].add(k)

        res[a] = len(m[a])
    return res

def polar_coords(m, x):
    pol = {}
    for k in m:
        if k==x:
            continue
        a = (k[0]-x[0], k[1]-x[1])
        norm = a[0]**2 + a[1]**2
        inv_tan = -inf if a[0]==0 else a[1]/a[0]
        h = 1 if a[0]<0 else -1 if a[0]>0 else -1 if a[1]<0 else 1
        if (h, inv_tan) in pol:
            pol[h,inv_tan].append(a)
        else:
            pol[h,inv_tan] = [a]
    for k in pol:
        pol[k] = sorted(pol[k], key=lambda a: a[0]**2 + a[1]**2)
    return pol



def part_one(m):
    res = sights(m)
    return max(res, key=lambda x:res[x]), max(res.values())

def part_two(m, x):
    pol = polar_coords(m, x)
    i = 0
    while True:
        for k in sorted(pol):
            ans = pol[k].pop(0)
            i += 1
            if i == 200:
                return (ans[0]+x[0])*100 + (ans[1]+x[1])

m = get_map(inp)
X, M = part_one(m)
print(f"Part one: {M}")
print(f"Part two: {part_two(m, X)}")
