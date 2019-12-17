import numpy as np
import re
from math import ceil

inp = open('input.txt').read().strip().split('\n')

def parse(inp):
    reactions = {}
    reg = re.compile(r'(\d+)[,\s]*(\w+)')
    for x in inp:
        r = reg.findall(x)
        r = list(map(lambda x: (int(x[0]),x[1]), r))
        reactions[r[-1]] = r[:-1]
    return reactions

def get_ore(reactions):
    oreac = {}
    for x,l in reactions.items():
        if len(l) == 1 and l[0][1]=='ORE':
            oreac[x] = l[0]
    return oreac

def find_reac(reactions, chem):
    for x in reactions:
        if x[1]==chem:
            return x, reactions[x]

def ore_price(ore_buy, oreac):
    ans = 0
    for x in ore_buy:
        r, orr = find_reac(oreac, x)
        num = ceil(ore_buy[x]/r[0])
        ans += num*orr[0]
    return ans

def fuel_price(inp, fuel_quantity=1):
    reactions = parse(inp)
    oreac = get_ore(reactions)
    oreac_list = [x[1] for x in oreac]
    reacs = {'FUEL': fuel_quantity}
    buy = {x[1]: [] for x in reactions}
    while reacs:
        next_reacs = {}
        for r in reacs:
            if r in oreac_list:
                pass
            else:
                buy[r].append(reacs[r])
                x, new_reac = find_reac(reactions, r)
                num = ceil(reacs[r] / x[0])
                for rr in new_reac:
                    if rr[1] in next_reacs:
                        next_reacs[rr[1]] += num*rr[0]
                    else:
                        next_reacs[rr[1]] = num*rr[0]
        reacs = next_reacs
    needs = {x: sum(buy[x]) for x in buy}
    while True:
        stocks = {x[1]: 0 for x in reactions}
        reacs = {'FUEL': fuel_quantity}
        ore_buy = {x: 0 for x in oreac_list}
        while reacs:
            next_reacs = {}
            for r in reacs:
                if r in oreac_list:
                    ore_buy[r] += reacs[r]
                elif r in stocks and reacs[r]<=stocks[r]:
                    stocks[r] -= reacs[r]
                    pass
                else:
                    x, new_reac = find_reac(reactions, r)
                    num = ceil(needs[r] / x[0])
                    for rr in new_reac:
                        if rr[1] in next_reacs:
                            next_reacs[rr[1]] += num*rr[0]
                        else:
                            next_reacs[rr[1]] = num*rr[0]
                    stocks[r] += needs[r]-reacs[r]

            reacs = next_reacs
        if all([stocks[x]==0 for x in stocks]):
            return ore_price(ore_buy, oreac)
        needs = {x: needs[x]-stocks[x] for x in buy}

def part_one(inp):
    return fuel_price(inp, 1)

def part_two(inp):
    trillion = 1000000000000
    a,b = 0, 100*trillion//part_one(inp) + 1
    while True:
        m = (a+b)//2
        x = fuel_price(inp, m)
        if x == trillion or m in [a,b]:
            return m
        elif x > trillion:
            b = m
        else:
            a = m

print(f"Part one: {part_one(inp)}")
print(f"Part two: {part_two(inp)}")
