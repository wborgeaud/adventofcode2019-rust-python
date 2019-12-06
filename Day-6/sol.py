inp = open('input.txt').read().strip().splitlines()
inp = [x.split(')') for x in inp]

def get_tree(inp):
    tree = {}
    parent = {}
    for x,y in inp:
        if x in tree:
            tree[x].append(y)
        else:
            tree[x] = [y]
        if y not in tree:
            tree[y] = []
        parent[y] = x
    return tree, parent

def count(x, current, total, tree):
    total[0] += current
    for y in tree[x]:
        count(y, current+1, total, tree)
    return total[0]

def get_parents(x, parent):
    parents = []
    y = parent[x]
    while y != 'COM':
        parents.append(y)
        y = parent[y]
    return parents

def part_one():
    tree, _ = get_tree(inp)
    return count('COM', 0, [0], tree)

def part_two():
    _, parent = get_tree(inp)
    parents_YOU = get_parents('YOU', parent)
    parents_SAN = get_parents('SAN', parent)
    for x in parents_YOU:
        if x in parents_SAN:
            common = x
            break
    return parents_YOU.index(x) + parents_SAN.index(x)


print(f"Part one: {part_one()}")
print(f"Part two: {part_two()}")