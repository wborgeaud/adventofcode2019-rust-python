def get_fuel(x):
    fuel = x//3 - 2
    return fuel if fuel > 0 else 0
def get_full_fuel(x):
    fuel = get_fuel(x)
    return fuel + get_full_fuel(fuel) if fuel > 0 else fuel

with open('input.txt') as f:
    modules = [int(x) for x in f.readlines()]

def part_one():
    return sum([get_fuel(x) for x in modules])

def part_two():
    return sum([get_full_fuel(x) for x in modules])

print(f"Part one: {part_one()}")
print(f"Part two: {part_two()}")
