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

fn get_map(program: &[isize]) -> Vec<Vec<isize>> {
    let (_input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let mut c = Computer::new(program.to_vec(), input_receiver, output_sender);
    thread::spawn(move || {
        c.run();
    });
    let mut res = vec![Vec::new()];
    loop {
        let out;
        let tout = output_receiver.recv();
        if let Ok(x) = tout {
            out = x;
        } else {
            break;
        }
        if out == 10 {
            res.push(Vec::new());
        } else {
            res.last_mut().unwrap().push(out);
        }
    }
    res.iter().filter(|l| !l.is_empty()).cloned().collect()
}

fn find_inter(map: &Vec<Vec<isize>>) -> Vec<(usize, usize)> {
    let mut inter = Vec::new();
    for i in 1..map.len()-1 {
        for j in 1..map[0].len()-1 {
            if map[i][j]==35 && 
            map[i-1][j]==35 &&
            map[i+1][j]==35 &&
            map[i][j-1]==35 &&
            map[i][j+1]==35 {
                inter.push((i,j));
            }

        }
    }
    inter
}

fn solve_it(program: &[isize]) -> isize {
    let routines = String::from("A,B,B,C,C,A,B,B,C,A");
    let ra = String::from("R,4,R,12,R,10,L,12");
    let rb = String::from("L,12,R,4,R,12");
    let rc = String::from("L,12,L,8,R,10");

    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let mut c = Computer::new(program.to_vec(), input_receiver, output_sender);
    thread::spawn(move || {
        c.run();
    });
    for s in [routines, ra, rb, rc].into_iter() {
        for c in s.bytes() {
            input_sender.send(c as isize).unwrap();
        }
        input_sender.send(10isize).unwrap();
    }
    input_sender.send(110isize).unwrap();
    input_sender.send(10isize).unwrap();

    let mut out = 0;
    loop {
        let tout = output_receiver.recv();
        if let Ok(x) = tout {
            out = x;
        } else {
            break;
        }
    }
    out
}

fn part_one(program: &[isize]) -> usize {
    let map = get_map(program);
    let inter = find_inter(&map);
    inter.iter().map(|(x,y)| x*y).sum()
}

fn part_two(init_program: &[isize]) -> isize {
    let mut program = init_program.to_vec();
    program[0] = 2;
    solve_it(&program)
}

fn main() {
    let program = get_input("../input.txt");
    println!("Part one: {}", part_one(&program.clone()));
    println!("Part two: {}", part_two(&program));
}
