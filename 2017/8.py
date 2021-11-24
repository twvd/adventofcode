import re

class Processor:
    def __init__(self):
        self.registers = {}
        self.rxInstruction = re.compile('(\w*) (dec|inc) ([0-9-]*) if (\w*) (<|<=|>=|>|!=|==) ([0-9-]*)')
        self.highestValue = 0

    def checkCondition(self, left, operator, right):
        # Resolve register
        if left not in self.registers.keys():
            self.registers[left] = 0
        left = self.registers[left]
        right = int(right)

        if operator == '==':
            return (left == right)
        elif operator == '!=':
            return (left != right)
        elif operator == '<':
            return (left < right)
        elif operator == '>':
            return (left > right)
        elif operator == '<=':
            return (left <= right)
        elif operator == '>=':
            return (left >= right)
        raise
    
    def execute(self, instruction):
        assignReg, assignOp, assignValue, condReg, condOp, condValue = self.rxInstruction.findall(instruction)[0]
    
        if (self.checkCondition(condReg, condOp, condValue)):
            if assignReg not in self.registers.keys():
                self.registers[assignReg] = 0
            if (assignOp == 'inc'):
                self.registers[assignReg] += int(assignValue)
            if (assignOp == 'dec'):
                self.registers[assignReg] -= int(assignValue)

            if (self.registers[assignReg] > self.highestValue):
                self.highestValue = self.registers[assignReg]

proc = Processor()

with open('8.txt') as f:
    for line in f.readlines():
        proc.execute(line)

print("Answer #1: %d" % max(proc.registers.values()))
print("Answer #2: %d" % proc.highestValue)