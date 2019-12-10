import time
class Computer:
    def __init__(self, memory, inputs):
        self.memory = memory + 10000*[0]
        self.ip = 0
        self.inputs = inputs
        self.outputs = []
        self.finished = False
        self.relative_base = 0
    def run(self):
        while not self.step():
            pass
        return self.outputs
    
    def step(self):
        code = self.memory[self.ip]
        if code % 100 == 99:
            self.finished = True
            return True
        elif code % 100 == 1:
            self.add(code)
        elif code % 100 == 2:
            self.multiply(code)
        elif code % 100 == 3:
            self.read_input(code)
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
        elif code % 100 == 9:
            self.adjust_relbase(code)
        else:
            raise Exception("Invalid opcode")

    def get_value(self, address, mode):
        if mode == 0:
            return self.memory[self.memory[address]]
        elif mode == 1:
            return self.memory[address]
        elif mode == 2:
            return self.memory[self.memory[address]+self.relative_base]
        else:
            raise Exception("Invalid parameter mode")

    def set_value(self, value, address, mode):
        if mode == 0:
            self.memory[self.memory[address]] = value
        elif mode == 2:
            self.memory[self.memory[address]+self.relative_base] = value
        else:
            raise Exception("Invalid parameter mode for setting")

    
    def add(self, code):
        params_mode = list(map(int,reversed(list(f"{code:05}"[:-2]))))
        sum_ = 0
        for i in range(2):
            sum_ += self.get_value(self.ip+i+1, params_mode[i])
        self.set_value(sum_, self.ip+3, params_mode[2])
        self.ip += 4

    def multiply(self, code):
        params_mode = list(map(int,reversed(list(f"{code:05}"[:-2]))))
        mul = 1
        for i in range(2):
            mul *= self.get_value(self.ip+i+1, params_mode[i])
        self.set_value(mul, self.ip+3, params_mode[2])
        self.ip += 4

    def read_input(self, code):
        params_mode = list(map(int,reversed(list(f"{code:03}"[:-2]))))
        x = self.inputs.pop(0)
        self.set_value(x, self.ip+1, params_mode[0])
        self.ip += 2

    def output(self, code):
        params_mode = list(map(int,reversed(list(f"{code:03}"[:-2]))))
        out = self.get_value(self.ip+1, params_mode[0])
        self.outputs.append(out)
        self.ip += 2
    
    def jmp(self, code):
        params_mode = list(map(int,reversed(list(f"{code:04}"[:-2]))))
        test_address = [0, 0]
        for i in range(2):
            test_address[i] = self.get_value(self.ip+i+1, params_mode[i])
        if test_address[0]:
            self.ip = test_address[1]
        else:
            self.ip += 3

    def jmpe(self, code):
        params_mode = list(map(int,reversed(list(f"{code:04}"[:-2]))))
        test_address = [0, 0]
        for i in range(2):
            test_address[i] = self.get_value(self.ip+i+1, params_mode[i])
        if test_address[0]==0:
            self.ip = test_address[1]
        else:
            self.ip += 3
    
    def lt(self, code):
        params_mode = list(map(int,reversed(list(f"{code:05}"[:-2]))))
        operands = [0,0]
        for i in range(2):
            operands[i] = self.get_value(self.ip+i+1, params_mode[i])
        out = 1 if operands[0] < operands[1] else 0
        self.set_value(out, self.ip+3, params_mode[2])
        self.ip += 4

    def equals(self, code):
        params_mode = list(map(int,reversed(list(f"{code:05}"[:-2]))))
        operands = [0,0]
        for i in range(2):
            operands[i] = self.get_value(self.ip+i+1, params_mode[i])
        out = 1 if operands[0] == operands[1] else 0
        self.set_value(out, self.ip+3, params_mode[2])
        self.ip += 4

    def adjust_relbase(self, code):
        params_mode = list(map(int,reversed(list(f"{code:03}"[:-2]))))
        value = self.get_value(self.ip+1, params_mode[0])
        self.relative_base += value
        self.ip += 2

