use std::{fs, sync::mpsc, thread};
mod computer;
use computer::Computer;

fn get_input(fp: &str) -> Vec<isize> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn get_coords(x: isize, y: isize, program: &[isize]) -> bool {
    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let mut c = Computer::new(program.to_vec(), input_receiver, output_sender);
    thread::spawn(move || {
        c.run();
    });
    input_sender.send(x).unwrap();
    input_sender.send(y).unwrap();
    let out = output_receiver.recv().unwrap();
    out == 1
}

fn find_max_y(x: isize, program: &[isize]) -> isize {
    let (mut y, mut Y) = (0, ((x as f64) / 0.6) as isize);
    loop {
        let m = (y + Y) / 2;
        if get_coords(x, m, program) {
            Y = m;
        } else {
            y = m;
        }
        if y == Y - 1 {
            return Y;
        }
    }
}

fn is_good(x: isize, y: isize, program: &[isize]) -> bool {
    get_coords(x, y, program)
        && get_coords(x, y + 99, program)
        && get_coords(x - 99, y + 99, program)
        && get_coords(x - 99, y, program)
}

fn part_one(program: &[isize]) -> usize {
    let mut ans = 0;
    for x in 0..50 {
        for y in 0..50 {
            ans += if get_coords(x, y, program) { 1 } else { 0 };
        }
    }
    ans
}

fn part_two(program: &[isize]) -> isize {
    let (mut x, mut X) = (0, 10000);
    loop {
        let m = (x + X) / 2;
        let y = find_max_y(m, program);
        if is_good(m, y, program) {
            X = m;
        } else {
            x = m;
        }
        if x == X - 1 {
            return (X - 99) * 10_000 + find_max_y(X, program);
        }
    }
}

fn main() {
    let program = get_input("../input.txt");
    println!("Part one: {}", part_one(&program.clone()));
    println!("Part two: {}", part_two(&program));
}
