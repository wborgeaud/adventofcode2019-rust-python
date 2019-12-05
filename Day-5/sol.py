program = list(map(int,open('input.txt').read().split(',')))

class Computer:
    def __init__(self, memory, input_):
        self.memory = memory
        self.ip = 0
        self.input = input_
    
    def run(self):
        while not self.step():
            pass
    
    def step(self):
        code = self.memory[self.ip]
        if code % 100 == 99:
            return True
        elif code % 100 == 1:
            self.add(code)
        elif code % 100 == 2:
            self.multiply(code)
        elif code % 100 == 3:
            self.read_input()
        elif code % 100 == 4:
            self.output(code)
        elif code % 100 == 5:
            self.jmp(code)
        elif code % 100 == 6:
            self.jmpe(code)
        elif code % 100 == 7:
            self.lt(code)
        elif code % 100 == 8:
            self.equals(code)
        else:
            raise Exception("Invalid opcode")
    
    def add(self, code):
        params_mode = list(map(int,reversed(list(f"{code:05}"[:-2]))))
        sum_ = 0
        for i in range(2):
            if params_mode[i] == 0:
                sum_ += self.memory[self.memory[self.ip+i+1]]
            elif params_mode[i] == 1:
                sum_ += self.memory[self.ip+i+1]
            else:
                raise Exception("Invalid parameter mode")
        if params_mode[2] == 0:
            self.memory[self.memory[self.ip+3]] = sum_
        else:
            raise Exception("Invalid parameter mode for return value")
        self.ip += 4

    def multiply(self, code):
        params_mode = list(map(int,reversed(list(f"{code:05}"[:-2]))))
        mul = 1
        for i in range(2):
            if params_mode[i] == 0:
                mul *= self.memory[self.memory[self.ip+i+1]]
            elif params_mode[i] == 1:
                mul *= self.memory[self.ip+i+1]
            else:
                raise Exception("Invalid parameter mode")
        if params_mode[2] == 0:
            self.memory[self.memory[self.ip+3]] = mul
        else:
            raise Exception("Invalid parameter mode for return value")
        self.ip += 4

    def read_input(self):
        self.memory[self.memory[self.ip+1]] = self.input
        self.ip += 2

    def output(self, code):
        params_mode = list(map(int,reversed(list(f"{code:03}"[:-2]))))
        if params_mode[0] == 0:
            out = self.memory[self.memory[self.ip+1]]
        elif params_mode[0] == 1:
            out = self.memory[self.ip+1]
        else:
            raise Exception("Invalid parameter mode")
        print(f"Output: {out}")
        self.ip += 2
    
    def jmp(self, code):
        params_mode = list(map(int,reversed(list(f"{code:04}"[:-2]))))
        test_address = [0, 0]
        for i in range(2):
            if params_mode[i]==0:
                test_address[i] = self.memory[self.memory[self.ip+i+1]]
            elif params_mode[i]==1:
                test_address[i] = self.memory[self.ip+i+1]
            else:
                raise Exception("Invalid parameter mode")

        if test_address[0]:
            self.ip = test_address[1]
        else:
            self.ip += 3

    def jmpe(self, code):
        params_mode = list(map(int,reversed(list(f"{code:04}"[:-2]))))
        test_address = [0, 0]
        for i in range(2):
            if params_mode[i]==0:
                test_address[i] = self.memory[self.memory[self.ip+i+1]]
            elif params_mode[i]==1:
                test_address[i] = self.memory[self.ip+i+1]
            else:
                raise Exception("Invalid parameter mode")

        if test_address[0]==0:
            self.ip = test_address[1]
        else:
            self.ip += 3
    
    def lt(self, code):
        params_mode = list(map(int,reversed(list(f"{code:05}"[:-2]))))
        operands = [0,0]
        for i in range(2):
            if params_mode[i] == 0:
                operands[i] = self.memory[self.memory[self.ip+i+1]]
            elif params_mode[i] == 1:
                operands[i] = self.memory[self.ip+i+1]
            else:
                raise Exception("Invalid parameter mode")
        out = 1 if operands[0] < operands[1] else 0
        if params_mode[2] == 0:
            self.memory[self.memory[self.ip+3]] = out
        else:
            raise Exception("Invalid parameter mode for return value")
        self.ip += 4

    def equals(self, code):
        params_mode = list(map(int,reversed(list(f"{code:05}"[:-2]))))
        operands = [0,0]
        for i in range(2):
            if params_mode[i] == 0:
                operands[i] = self.memory[self.memory[self.ip+i+1]]
            elif params_mode[i] == 1:
                operands[i] = self.memory[self.ip+i+1]
            else:
                raise Exception("Invalid parameter mode")
        out = 1 if operands[0] == operands[1] else 0
        if params_mode[2] == 0:
            self.memory[self.memory[self.ip+3]] = out
        else:
            raise Exception("Invalid parameter mode for return value")
        self.ip += 4

def part_one():
    computer = Computer(program.copy(), 1)
    computer.run()

def part_two():
    computer = Computer(program.copy(), 5)
    computer.run()

print("Running part one")
part_one()
print("Running part two")
part_two()