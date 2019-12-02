use std::{cmp::max, error::Error, fmt, fs};

fn get_input(fp: &str) -> Vec<usize> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct SegfaultError;
impl fmt::Display for SegfaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Segfault")
    }
}
impl Error for SegfaultError {}

fn opcode(ip: usize, program: &mut [usize]) -> Result<usize, SegfaultError> {
    let a = program[ip + 1];
    let b = program[ip + 2];
    let c = program[ip + 3];
    if max(a, max(b, c)) >= program.len() {
        Err(SegfaultError)
    } else {
        match program[ip] {
            1 => program[c] = program[a] + program[b],
            2 => program[c] = program[a] * program[b],
            _ => return Err(SegfaultError),
        };
        Ok(ip + 4)
    }
}

fn run(program: &mut [usize]) -> Result<(), SegfaultError> {
    let mut ip = 0;
    while program[ip] != 99 {
        ip = opcode(ip, program)?;
    }
    Ok(())
}

fn prepare(program: &mut [usize], a: usize, b: usize) {
    program[1] = a;
    program[2] = b;
}

fn part_one(program: &[usize]) -> usize {
    let mut prog_copy = vec![0; program.len()];
    prog_copy.copy_from_slice(program);
    prepare(&mut prog_copy, 12, 2);
    run(&mut prog_copy).expect("Segfault");
    prog_copy[0]
}

fn part_two(program: &[usize]) -> Option<usize> {
    for a in 0..100 {
        for b in 0..100 {
            let mut prog_copy = vec![0; program.len()];
            prog_copy.copy_from_slice(program);
            prepare(&mut prog_copy, a, b);
            match run(&mut prog_copy) {
                Ok(_) => {
                    if prog_copy[0] == 19_690_720 {
                        return Some(a * 100 + b);
                    }
                }
                Err(SegfaultError) => continue
            }
        }
    }
    None
}

fn main() {
    let program = get_input("../input.txt");
    println!("Part one: {}", part_one(&program));
    println!("Part two: {}", part_two(&program).unwrap());
}
