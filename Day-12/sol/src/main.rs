use num_integer::Integer;
use regex::Regex;
use std::fs;


fn parse(s: &str) -> Position {
    let re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();
    let bam = re.captures(s).unwrap();
    let boom = (1..=3).map(|i| bam.get(i).unwrap().as_str().to_string());
    let boom: Vec<isize> = boom.map(|x| x.parse().unwrap()).collect();
    Position(boom[0], boom[1], boom[2])
}

fn get_input(fp: &str) -> Vec<Position> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .trim_end()
        .split('\n')
        .map(parse)
        .collect()
}

#[derive(Debug, Copy, Clone)]
struct Position(isize, isize, isize);
#[derive(Debug, Copy, Clone)]
struct Velocity(isize, isize, isize);

impl Position {
    fn add(&mut self, v: &Velocity) {
        self.0 += v.0;
        self.1 += v.1;
        self.2 += v.2;
    }
    fn potential(&self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
    fn get(&self, i: isize) -> isize {
        match i {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!(),
        }
    }
}
impl Velocity {
    fn add(&mut self, v: &Velocity) {
        self.0 += v.0;
        self.1 += v.1;
        self.2 += v.2;
    }
    fn kinetic(&self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
    fn zeros() -> Self {
        Velocity(0, 0, 0)
    }
    fn get(&self, i: isize) -> isize {
        match i {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!(),
        }
    }
}

fn sign(a: isize, b: isize) -> isize {
    if a < b {
        1
    } else if a > b {
        -1
    } else {
        0
    }
}

fn gravity(pos: &[Position]) -> Vec<Velocity> {
    let mut ans = Vec::new();
    for p in pos.iter() {
        let mut v = [0; 3];
        for pp in pos.iter() {
            v[0] += sign(p.0, pp.0);
            v[1] += sign(p.1, pp.1);
            v[2] += sign(p.2, pp.2);
        }
        ans.push(Velocity(v[0], v[1], v[2]));
    }
    ans
}

fn energy(pos: &[Position], v: &[Velocity]) -> isize {
    let mut res = 0;
    for i in 0..4 {
        res += pos[i].potential() * v[i].kinetic();
    }
    res
}

fn step(pos: &mut [Position], v: &mut [Velocity]) {
    let g = gravity(pos);
    (0..4).for_each(|i| v[i].add(&g[i]));
    (0..4).for_each(|i| pos[i].add(&v[i]));
}

fn part_one(init_pos: &[Position]) -> isize {
    let mut pos = init_pos.to_vec();
    let mut v = vec![Velocity::zeros(); 4];
    for _ in 0..1000 {
        step(&mut pos, &mut v);
    }
    energy(&pos, &v)
}

fn cycles(init_pos: &[Position], index: isize) -> usize {
    let mut pos = init_pos.to_vec();
    let mut v = vec![Velocity::zeros(); 4];
    let mut res = 1;
    step(&mut pos, &mut v);
    while !(0..4).all(|i| v[i].get(index) == 0)
    {
        step(&mut pos, &mut v);
        res += 1;
    }
    2 * res
}

fn part_two(init_pos: &[Position]) -> usize {
    let cycs: Vec<usize> = (0..3).map(|i| cycles(init_pos, i)).collect();
    cycs.iter().fold(1, |acc, x| acc.lcm(x))
}

fn main() {
    let pos = get_input("../input.txt");
    println!("Part one: {}", part_one(&pos));
    println!("Part two: {}", part_two(&pos));
}
