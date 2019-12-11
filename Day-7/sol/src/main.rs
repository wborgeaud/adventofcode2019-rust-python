use itertools::Itertools;
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

fn part_one(program: &[isize]) -> isize {
    let mut max = 0;
    for seq in (0..5).permutations(5) {
        let mut inp = 0;
        let mut input_senders = Vec::new();
        let mut output_receivers = Vec::new();
        for i in 0..5 {
            let (input_sender, input_receiver) = mpsc::channel();
            input_senders.push(input_sender);
            let (output_sender, output_receiver) = mpsc::channel();
            output_receivers.push(output_receiver);
            let mut c = Computer::new(program.to_vec(), input_receiver, output_sender);
            thread::spawn(move || {
                c.run();
            });
            input_senders[i].send(seq[i]).unwrap();
            input_senders[i].send(inp).unwrap();
            inp = output_receivers[i].recv().unwrap();
        }
        if inp > max {
            max = inp;
        }
    }
    max
}

fn part_two(program: &[isize]) -> isize {
    let mut max = 0;
    for seq in (5..10).permutations(5) {
        let mut input_senders = Vec::new();
        let mut output_receivers = Vec::new();
        for i in 0..5 {
            let (input_sender, input_receiver) = mpsc::channel();
            input_senders.push(input_sender);
            let (output_sender, output_receiver) = mpsc::channel();
            output_receivers.push(output_receiver);
            let mut c = Computer::new(program.to_vec(), input_receiver, output_sender);
            thread::spawn(move || {
                c.run();
            });
            input_senders[i].send(seq[i]).unwrap();

        }
        let mut inp = 0;
        let mut i = 0;
        loop {
            let j = i%5;
            if input_senders[j].send(inp).is_err() {
                break;
            }             
            if let Ok(x) = output_receivers[j].recv() {
                inp = x;
            } else {
                break;
            }
            i += 1;
        }
        if inp > max {
            max = inp;
        }
    }
    max
}

fn main() {
    let program = get_input("../input.txt");
    println!("Part one: {}", part_one(&program));
    println!("Part two: {}", part_two(&program));
}
