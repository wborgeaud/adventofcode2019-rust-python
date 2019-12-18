use std::{fs, collections::{HashMap, HashSet}, sync::mpsc, thread, iter::FromIterator};
mod computer;
use computer::Computer;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Cave {
    Wall,
    Floor,
    Oxygen,
}
impl Cave {
    fn from(i: isize) -> Self{
        match i {
            0 => Self::Wall,
            1 => Self::Floor,
            2 => Self::Oxygen,
            _ => panic!("Invalid code")
        }
    }
}

type Map = HashMap<Position, (Cave, usize)>;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position(isize, isize);
impl Position {
    fn change(&self, code: isize) -> Self {
        let mut new = self.clone();
        match code {
            1 => new.1 += 1,
            2 => new.1 -= 1,
            3 => new.0 += 1,
            4 => new.0 -= 1,
            _ => panic!("Invalid code")
        }
        new
    }
}

fn opposite(code: &isize) -> isize {
    match code {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("Invalid code")
    }
}

fn get_input(fp: &str) -> Vec<isize> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn try_path(path: &[isize], program: &[isize]) -> isize {
    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let mut c = Computer::new(program.to_vec(), input_receiver, output_sender);
    thread::spawn(move || {
        c.run();
    });
    let mut out = 0;
    for &v in path.iter() {
        input_sender.send(v).unwrap();
        out = output_receiver.recv().unwrap();
    }
    out
}

fn get_map(program: &[isize]) -> Map {
    let mut map = HashMap::new();
    map.insert(Position(0,0), (Cave::Floor, 0));
    let mut queue = vec![(Vec::new(), Position(0,0))];
    while !queue.is_empty() {
        let v = queue.remove(0);
        'inner: for d in 1..=4 {
            if v.0.len()>0 && d == opposite(v.0.last().unwrap()) {
                continue 'inner;
            }
            let mut path = v.0.clone();
            path.push(d);
            let out = try_path(&path, program);
            let nv = v.1.change(d);
            map.insert(nv, (Cave::from(out), v.0.len()+1));
            match out {
                0 => continue 'inner,
                _ => queue.push((path, nv)),
            }
        }
    }
    map
}

fn bfs(start: Position, map: &Map) -> usize {
    let mut queue = vec![(start, 0)];
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut maxlen = 0;
    while !queue.is_empty() {
        let v = queue.remove(0);
        if v.1 > maxlen {
            maxlen = v.1;
        }
        'inner: for d in 1..=4 {
            let nv = v.0.change(d);
            if seen.contains(&nv) {
                continue 'inner;
            } else {
                seen.insert(nv);
            }
            let out = map.get(&nv).unwrap().0;
            match out {
                Cave::Wall => continue 'inner,
                _ => queue.push((nv, v.1+1))
            }
        }
    }
    maxlen
}

fn part_one(map: &Map) -> Option<usize> {
    for x in map.values() {
        if x.0 == Cave::Oxygen {
            return Some(x.1);
        }
    }
    None
}

fn part_two(map: &Map) -> usize {
    let mut start = Position(0,0);
    for (k, v) in map.iter() {
        if v.0  == Cave::Oxygen {
            start = *k;
            break;
        }
    }
    bfs(start, map)
}

fn main() {
    let program = get_input("../input.txt");
    let map = get_map(&program);
    println!("Part one: {}", part_one(&map).unwrap());
    println!("Part two: {}", part_two(&map));
}