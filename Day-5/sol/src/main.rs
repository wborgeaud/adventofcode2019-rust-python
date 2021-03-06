use std::fs;

fn get_input(fp: &str) -> Vec<isize> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

struct Computer {
    memory: Vec<isize>,
    ip: usize,
    input: isize,
}

impl Computer {
    fn new(memory: Vec<isize>, input: isize) -> Self {
        Computer {
            memory,
            ip: 0,
            input,
        }
    }

    fn get(&self, x: usize) -> isize {
        self.memory[self.memory[x] as usize]
    }

    fn set(&mut self, x: usize, v: isize) {
        let ind = self.memory[x] as usize;
        self.memory[ind] = v;
    }

    fn get_params_mode(code: isize, num: usize) -> Vec<usize> {
        let s;
        match num {
            3 => s = format!("{:03}", code),
            4 => s = format!("{:04}", code),
            5 => s = format!("{:05}", code),
            _ => panic!(),
        }
        let params_mode: Vec<usize> = s
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .rev()
            .collect();
        Vec::from(&params_mode[2..])
    }

    fn run(&mut self) {
        loop {
            if let Some(x) = self.step() {
                return;
            }
        }
    }

    fn step(&mut self) -> Option<()> {
        let code = self.memory[self.ip];
        match code % 100 {
            99 => return Some(()),
            1 => self.add(code),
            2 => self.multiply(code),
            3 => self.read_input(),
            4 => self.output(code),
            5 => self.jmp(code),
            6 => self.jmpe(code),
            7 => self.lt(code),
            8 => self.equals(code),
            _ => panic!("Invalid opcode"),
        }
        None
    }

    fn add(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 5);
        let mut sum = 0;
        for i in 0..2 {
            match params_mode[i] {
                0 => sum += self.get(self.ip + i + 1),
                1 => sum += self.memory[self.ip + i + 1],
                _ => panic!("Invalid parameter mode"),
            }
        }
        match params_mode[2] {
            0 => self.set(self.ip + 3, sum),
            _ => panic!("Invalid parameter mode"),
        }
        self.ip += 4;
    }

    fn multiply(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 5);
        let mut mul = 1;
        for i in 0..2 {
            match params_mode[i] {
                0 => mul *= self.get(self.ip + i + 1),
                1 => mul *= self.memory[self.ip + i + 1],
                _ => panic!("Invalid parameter mode"),
            }
        }
        match params_mode[2] {
            0 => self.set(self.ip + 3, mul),
            _ => panic!("Invalid parameter mode"),
        }
        self.ip += 4;
    }

    fn read_input(&mut self) {
        self.set(self.ip + 1, self.input);
        self.ip += 2;
    }

    fn output(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 3);
        let mut out;
        match params_mode[0] {
            0 => out = self.get(self.ip + 1),
            1 => out = self.memory[self.ip + 1],
            _ => panic!("Invalid parameter mode"),
        }
        println!("Output: {}", out);
        self.ip += 2;
    }

    fn jmp(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 4);
        let mut test_address = [0, 0];
        for i in 0..2 {
            match params_mode[i] {
                0 => test_address[i] = self.get(self.ip + i + 1),
                1 => test_address[i] = self.memory[self.ip + i + 1],
                _ => panic!("Invalid parameter mode"),
            }
        }
        if test_address[0] != 0 {
            self.ip = test_address[1] as usize;
        } else {
            self.ip += 3;
        }
    }

    fn jmpe(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 4);
        let mut test_address = [0, 0];
        for i in 0..2 {
            match params_mode[i] {
                0 => test_address[i] = self.get(self.ip + i + 1),
                1 => test_address[i] = self.memory[self.ip + i + 1],
                _ => panic!("Invalid parameter mode"),
            }
        }
        if test_address[0] == 0 {
            self.ip = test_address[1] as usize;
        } else {
            self.ip += 3;
        }
    }

    fn lt(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 5);
        let mut operands = [0, 0];
        for i in 0..2 {
            match params_mode[i] {
                0 => operands[i] = self.get(self.ip + i + 1),
                1 => operands[i] = self.memory[self.ip + i + 1],
                _ => panic!("Invalid parameter mode"),
            }
        }
        let out = if operands[0] < operands[1] { 1 } else { 0 };
        match params_mode[2] {
            0 => self.set(self.ip + 3, out),
            _ => panic!("Invalid parameter mode"),
        }
        self.ip += 4;
    }

    fn equals(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 5);
        let mut operands = [0, 0];
        for i in 0..2 {
            match params_mode[i] {
                0 => operands[i] = self.get(self.ip + i + 1),
                1 => operands[i] = self.memory[self.ip + i + 1],
                _ => panic!("Invalid parameter mode"),
            }
        }
        let out = if operands[0] == operands[1] { 1 } else { 0 };
        match params_mode[2] {
            0 => self.set(self.ip + 3, out),
            _ => panic!("Invalid parameter mode"),
        }
        self.ip += 4;
    }
}

fn part_one(program: Vec<isize>) {
    let mut computer = Computer::new(program, 1);
    computer.run();
}

fn part_two(program: Vec<isize>) {
    let mut computer = Computer::new(program, 5);
    computer.run();
}

fn main() {
    let program = get_input("../input.txt");
    println!("Running part one");
    part_one(program.clone());
    println!("Running part two");
    part_two(program.clone());
}
