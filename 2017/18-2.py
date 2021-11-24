import string
import queue

class Processor:
    def __init__(self, pId, code):
        self.registers = {}
        for c in string.ascii_lowercase:
            self.registers[c] = 0
        
        self.code = code
        self.registers['p'] = pId
        self.pc = 0
        self.input = queue.Queue()
        self.output = queue.Queue()
        self.waiting = False

    def isLocked(self):        
        return self.input.empty() and self.waiting

    def hasOutput(self):
        return not self.output.empty()

    def run(self):
        while not self.step():
            pass
    
    def step(self):
        return self.execute(self.code[self.pc])

    def execute(self, code):        
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
            self.output.put(vleft)
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
            if self.input.empty():
                self.waiting = True
                return True
            self.waiting = False

            self.registers[left] = self.input.get()
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

prog0 = Processor(0, instructions)
prog1 = Processor(1, instructions)

answer2 = 0

while True:
    prog0.step()
    prog1.step()
    if prog0.hasOutput():
        v = prog0.output.get()
        prog1.input.put(v)

    if prog1.hasOutput():
        v = prog1.output.get()
        prog0.input.put(v)
        answer2 += 1

    if prog0.isLocked() and prog1.isLocked():
        break
print("Answer #2: %d" % answer2)