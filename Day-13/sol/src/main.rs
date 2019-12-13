use std::collections::HashMap;
use std::fs;
use std::sync::mpsc;
use std::thread;

mod computer;
use computer::Computer;

fn get_input(fp: &str) -> Vec<isize> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn make_tiles(program: &[isize]) -> HashMap<(isize, isize), isize> {
    let mut tiles: HashMap<(isize, isize), isize> = HashMap::new();
    let (_input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let mut c = Computer::new(program.to_vec(), input_receiver, output_sender);
    thread::spawn(move || {
        c.run();
    });
    'run: loop {
        let mut out = Vec::new();
        for _ in 0..3 {
            let x = output_receiver.recv();
            if x.is_err() {
                break 'run;
            }
            out.push(x.unwrap());
        }
        tiles.insert((out[0], out[1]), out[2]);
    }
    tiles
}

fn play(program: &[isize]) -> isize {
    let mut tiles: HashMap<(isize, isize), isize> = HashMap::new();
    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let mut c = Computer::new(program.to_vec(), input_receiver, output_sender);
    thread::spawn(move || {
        c.run();
    });
    let mut score = 0;
    input_sender.send(0);
    'run: loop {
        let mut out = Vec::new();
        for _ in 0..3 {
            let x = output_receiver.recv();
            if x.is_err() {
                break 'run;
            }
            out.push(x.unwrap());
        }
        if (out[0], out[1]) == (-1, 0) {
            score = out[2];
        } else {
            tiles.insert((out[0], out[1]), out[2]);
        }
        if out[2] == 4 && tiles.values().any(|&a| a == 3) {
            let paddle_pos = tiles.keys().find(|&a| tiles.get(a).unwrap() == &3).unwrap();
            if out[0] > paddle_pos.0 {
                input_sender.send(1);
            } else if out[0] < paddle_pos.0 {
                input_sender.send(-1);
            } else {
                input_sender.send(0);
            }
        }
    }
    score
}

fn part_one(program: &[isize]) -> usize {
    let tiles = make_tiles(program);
    tiles.values().filter(|&&v| v == 2).count()
}

fn part_two(init_program: &[isize]) -> isize {
    let mut program = init_program.to_vec();
    program[0] = 2;
    play(&program)
}

fn main() {
    let program = get_input("../input.txt");
    println!("Part one: {}", part_one(&program.clone()));
    println!("Part two: {}", part_two(&program));
}
