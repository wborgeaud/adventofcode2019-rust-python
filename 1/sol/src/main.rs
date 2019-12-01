use std::fs;

fn get_input(fp: &str) -> Vec<usize> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_fuel(x: usize) -> usize {
    let fuel = x / 3;
    if fuel > 2 {
        fuel - 2
    } else {
        0
    }
}

fn get_full_fuel(x: usize) -> usize {
    let fuel = get_fuel(x);
    if fuel > 0 {
        fuel + get_full_fuel(fuel)
    } else {
        fuel
    }
}

fn part_one(modules: &[usize]) -> usize {
    modules.iter().map(|&x| get_fuel(x)).sum()
}

fn part_two(modules: &[usize]) -> usize {
    modules.iter().map(|&x| get_full_fuel(x)).sum()
}

fn main() {
    let modules = get_input("../input.txt");
    println!("Part one: {}", part_one(&modules));
    println!("Part two: {}", part_two(&modules));
}
