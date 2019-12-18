use std::sync::mpsc::{Receiver, Sender};

pub struct Computer {
    memory: Vec<isize>,
    ip: usize,
    input_receiver: Receiver<isize>,
    output_sender: Sender<isize>,
    relative_base: isize,
    stop: bool
}

impl Computer {
    pub fn new(
        memory: Vec<isize>,
        input_receiver: Receiver<isize>,
        output_sender: Sender<isize>,
    ) -> Self {
        let mut nmem = memory.clone();
        nmem.append(&mut vec![0; 10000]);
        Computer {
            memory: nmem,
            ip: 0,
            input_receiver,
            output_sender,
            relative_base: 0,
            stop: false
        }
    }

    fn get(&self, address: usize, mode: usize) -> isize {
        match mode {
            0 => self.memory[self.memory[address] as usize],
            1 => self.memory[address],
            2 => self.memory[(self.relative_base + self.memory[address]) as usize],
            _ => panic!("Unknown parameter mode"),
        }
    }
    fn set(&mut self, value: isize, address: usize, mode: usize) {
        let p = self.memory[address];
        match mode {
            0 => self.memory[p as usize] = value,
            2 => self.memory[(self.relative_base + p) as usize] = value,
            _ => panic!("Unknown parameter mode"),
        };
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

    pub fn run(&mut self) {
        loop {
            if self.step().is_some() || self.stop {
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
            3 => self.read_input(code),
            4 => self.output(code),
            5 => self.jmp(code),
            6 => self.jmpe(code),
            7 => self.lt(code),
            8 => self.equals(code),
            9 => self.adjust_relbase(code),
            _ => panic!("Invalid opcode"),
        }
        None
    }

    fn add(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 5);
        let mut sum = 0;
        for i in 0..2 {
            sum += self.get(self.ip + i + 1, params_mode[i]);
        }
        self.set(sum, self.ip + 3, params_mode[2]);
        self.ip += 4;
    }

    fn multiply(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 5);
        let mut mul = 1;
        for i in 0..2 {
            mul *= self.get(self.ip + i + 1, params_mode[i]);
        }
        self.set(mul, self.ip + 3, params_mode[2]);
        self.ip += 4;
    }

    fn read_input(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 3);
        let bam = self.input_receiver.recv();
        if bam.is_err() {self.stop=true;return};
        let x = bam.unwrap();
        // let x = self.input_receiver.recv().unwrap();
        self.set(x, self.ip + 1, params_mode[0]);
        self.ip += 2;
    }

    fn output(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 3);
        let out;
        out = self.get(self.ip + 1, params_mode[0]);
        self.output_sender.send(out).unwrap();
        self.ip += 2;
    }

    fn jmp(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 4);
        let mut test_address = [0, 0];
        for i in 0..2 {
            test_address[i] = self.get(self.ip + i + 1, params_mode[i]);
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
            test_address[i] = self.get(self.ip + i + 1, params_mode[i]);
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
            operands[i] = self.get(self.ip + i + 1, params_mode[i]);
        }
        let out = if operands[0] < operands[1] { 1 } else { 0 };
        self.set(out, self.ip + 3, params_mode[2]);
        self.ip += 4;
    }

    fn equals(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 5);
        let mut operands = [0, 0];
        for i in 0..2 {
            operands[i] = self.get(self.ip + i + 1, params_mode[i]);
        }
        let out = if operands[0] == operands[1] { 1 } else { 0 };
        self.set(out, self.ip + 3, params_mode[2]);
        self.ip += 4;
    }

    fn adjust_relbase(&mut self, code: isize) {
        let params_mode = Self::get_params_mode(code, 3);
        self.relative_base += self.get(self.ip + 1, params_mode[0]);
        self.ip += 2;
    }
}
