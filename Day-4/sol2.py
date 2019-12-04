def is_good_one(n):
    sn = str(n)
    return (
        len(sn) == 6 and
        any([sn[i]==sn[i-1] for i in range(1, len(sn))]) and
        all([sn[i]>=sn[i-1] for i in range(1,len(sn))])
    )

def matches(s):
    res = []
    i, j = 0, 1
    while True:
        if j >= len(s):
            res.append(j-i)
            return res
        elif s[i] == s[j]:
            j += 1
        else:
            res.append(j-i)
            if j >= len(s):
                return res
            i, j = j, j+1
        

def is_good_two(n):
    sn = str(n)
    matchs = matches(sn)
    return (
        len(sn) == 6 and
        any([sn[i]==sn[i-1] for i in range(1, len(sn))]) and
        all([sn[i]>=sn[i-1] for i in range(1,len(sn))]) and 
        any([x==2 for x in matchs])
    )

def part_one(a, b):
    return sum([is_good_one(n) for n in range(a,b+1)])
def part_two(a, b):
    return sum([is_good_two(n) for n in range(a,b+1)])

inp = 206938,679128
print(f"Part one: {part_one(*inp)}")
print(f"Part two: {part_two(*inp)}")
