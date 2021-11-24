import string
import sys

class Processor:
    def __init__(self):
        self.registers = {}
        for c in string.ascii_lowercase:
            self.registers[c] = 0
        self.lastFrequency = 0
        self.pc = 0

    def part1(self, code):
        while not self.execute(code[self.pc]):
            pass
        print("Answer #1: %d" % self.lastFrequency)

    def execute(self, code):
        #print(code)
        #print(self.registers)
        spl = code.split()
        instr = spl[0]
        left = spl[1]

        if len(spl) > 2:
            right = spl[2]
        else:
            right = '0'

        if right.isalpha():
            vright = self.registers[right]
        else:
            vright = int(right)
        if left.isalpha():
            vleft = self.registers[left]
        else:
            vleft = int(left)

        if instr == "snd":
            self.lastFrequency = vleft
        elif instr == "set":
            self.registers[left] = vright
        elif instr == "add":
            self.registers[left] += vright
        elif instr == "mul":
            self.registers[left] *= vright
        elif instr == "mod":
            self.registers[left] %= vright
        elif instr == "rcv":
            if self.lastFrequency > 0:
                self.registers[left] = self.lastFrequency
                return True
        elif instr == "jgz":
            if vleft > 0:
                self.pc += vright
                return False
        else:
            raise NotImplementedError('Unknown instruction')

        self.pc += 1
        return False

with open('18.txt') as f:
    inp = f.readlines()

instructions = [i for i in map(str.strip, inp)]

p = Processor()
p.part1(instructions)