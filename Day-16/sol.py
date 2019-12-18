import numpy as np

inp = np.array(list(map(int,open('input.txt').read().strip())),dtype=np.int64)

def pattern(i):
        return np.repeat([0,1,0,-1],i)

def fft(a):
    n = len(a) 
    new = np.zeros(n, dtype=np.int64)
    for i in range(n):
        patt = pattern(i+1)
        patt = np.tile(patt, 1+(n+1)//len(patt))[1:n+1]
        new[i] = np.abs(np.dot(a, patt))%10
    return new

def fast_fft(a, offset, rep):
    if offset<len(a)/2: return
    a = a[offset:]
    for it in range(rep):
        s = a.sum()
        na = np.mod(np.abs(s-np.cumsum(a)+a),10)
        a = na
    return a[:8]

def part_one(inp):
    x = np.copy(inp)
    for _ in range(100):
        x = fft(x)
    return ''.join(map(str,x[:8]))

def part_two(inp):
    x = np.copy(inp)
    x = np.tile(x, 10000)
    offset = int(''.join(map(str,x[:7])))
    return ''.join(map(str,fast_fft(x, offset, 100)))


print(f"Part one: {part_one(inp)}")
print(f"Part two: {part_two(inp)}")
