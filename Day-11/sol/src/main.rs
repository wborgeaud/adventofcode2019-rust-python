use std::fs;
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

mod computer;
use computer::Computer;

fn get_input(fp: &str) -> Vec<isize> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn paint(program: &[isize], first_color: u8) -> HashMap<(isize, isize), u8> {
    let mut panels: HashMap<(isize, isize),u8> = HashMap::new();
    let (mut pos, mut last_pos) = ((0,0), (0,-1));
    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let mut c = Computer::new(program.to_vec(), input_receiver, output_sender);
    thread::spawn(move || {
        c.run();
    });
    loop {
        let current_color = panels.get(&pos).or_else(|| {
            if !panels.is_empty() {
                Some(&0u8)
            } else {
                Some(&first_color)
            }
        }).unwrap();
        if input_sender.send(*current_color as isize).is_err() {
            break;
        }             
        let new_color = output_receiver.recv();
        let direction = output_receiver.recv();
        if let (Err(_),Err(_)) = (new_color, direction) {
            break;
        }
        panels.insert(pos, new_color.unwrap() as u8);
        if direction.unwrap() == 1 {
            let old_pos = pos;
            pos = (pos.0 + pos.1 - last_pos.1, pos.1 - pos.0 + last_pos.0);
            last_pos = old_pos;
        } else {
            let old_pos = pos;
            pos = (pos.0 - pos.1 + last_pos.1, pos.1 + pos.0 - last_pos.0);
            last_pos = old_pos;
        }
    }
    panels
}

fn part_one(program: &[isize]) -> usize {
    let panels = paint(program, 0);
    panels.len()
}

fn part_two(program: &[isize]) -> String {
    let panels = paint(program, 1);
    let keys: Vec<(isize, isize)> = panels.keys().copied().collect();
    let (mx, Mx) = (keys.iter().map(|(i,_)| *i).min().unwrap(),keys.iter().map(|(i,_)| *i).max().unwrap());
    let (my, My) = (keys.iter().map(|(_,j)| *j).min().unwrap(),keys.iter().map(|(_,j)| *j).max().unwrap());
    let mut res = String::from("\n");
    for j in (my..=My).rev() {
        for i in mx..=Mx {
            if panels.get(&(i,j))==Some(&1) {
                res.push('⬛');
            } else {
                res.push(' ');
            }
        }
        res.push('\n');
    }
    res
}

fn main() {
    let program = get_input("../input.txt");
    println!("Part one: {}", part_one(&program));
    println!("Part two: {}", part_two(&program));
}
