use std::{fs, sync::mpsc, thread};
mod computer;
use computer::Computer;

const CODE1: &str = "NOT C J
AND D J
NOT A T
AND D T
OR T J
WALK
";
const CODE2: &str = "NOT C J
AND D J
AND H J
NOT A T
AND D T
OR T J
NOT B T
AND D T
OR T J
RUN
";

fn get_input(fp: &str) -> Vec<isize> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn run_program(code: &str, program: &[isize]) -> isize {
    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let mut c = Computer::new(program.to_vec(), input_receiver, output_sender);
    thread::spawn(move || {
        c.run();
    });
    for x in code.bytes() {
        input_sender.send(x as isize).unwrap();
    }
    let mut ans = 0;
    loop {
        let out = output_receiver.recv();
        if let Ok(x) = out {
            ans = x;
        } else {
            break;
        }
    }
    ans
}

fn part_one(program: &[isize]) -> isize {
    run_program(CODE1, program)
}

fn part_two(program: &[isize]) -> isize {
    run_program(CODE2, program)
}

fn main() {
    let program = get_input("../input.txt");
    println!("Part one: {}", part_one(&program.clone()));
    println!("Part two: {}", part_two(&program));
}