use std::fs;

mod computer;
use computer::Computer;

fn get_input(fp: &str) -> Vec<isize> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}


fn part_one(program: Vec<isize>) -> isize {
    let mut computer = Computer::new(program, 1);
    computer.run()[0]
}

fn part_two(program: Vec<isize>) -> isize {
    let mut computer = Computer::new(program, 2);
    computer.run()[0]
}

fn main() {
    let program = get_input("../input.txt");
    println!("Part one: {}", part_one(program.clone()));
    println!("Part two: {}", part_two(program.clone()));
}
