import numpy as np

inp = open('input.txt').read().strip()
#inp = '123456789012' 

def into_layers(inp, h, w):
    depth = len(inp)//(h*w)
    layers = np.zeros((h,w,depth))
    a = h*w
    for i,s in enumerate(inp):
        layers[(i%a)//w, (i%a)%w, i//a] = int(s)
    return layers

def remove_transparent(l):
    for x in l:
        if x != 2:
            return x

def part_one(layers):
    min_0 = np.argmin((layers==0).sum((0,1)))
    lay_0 = layers[:,:,min_0]
    return (lay_0==1).sum() * (lay_0==2).sum()

def part_two(layers):
    image = np.apply_along_axis(remove_transparent, 2, layers)
    ans = '\n'
    for row in image:
        ans += (''.join(['⬛' if x else ' ' for x in row])) 
        ans += '\n'
    return ans
layers = into_layers(inp, 6, 25)
print(f"Part one: {part_one(layers)}")
print(f"Part two: {part_two(layers)}")
