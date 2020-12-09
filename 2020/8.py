import copy

with open('8.txt') as f:
    inp = [l.strip() for l in f.readlines()]
    
def step(instr, pc, acc):
    opcode, val = instr.split(' ')
    if opcode == 'nop':
        pc += 1
    elif opcode == 'acc':
        pc += 1
        acc += int(val)
    elif opcode == 'jmp':
        pc += int(val)
    return pc, acc

def part1(prog):
    pcs_visited = []
    acc, pc = 0, 0

    while True:
        pc, acc = step(prog[pc], pc, acc)
        if pc in pcs_visited: 
            break
        pcs_visited.append(pc)
    return acc

def part2(prog):
    for i, v in enumerate(prog):
        if v[:3] == 'acc':
            continue
        newprog = copy.copy(prog)
        if v[:3] == 'jmp':
            newprog[i] = 'nop' + v[3:]
        if v[:3] == 'nop':
            newprog[i] = 'jmp' + v[3:]
        pc, acc = 0, 0
        for _ in range(10000):
            pc, acc = step(newprog[pc], pc, acc)
            if pc == len(newprog):
                print("Line %d/%d: %s" % (i, len(prog), v))
                return acc
    return -1

print("Answer #1: ", part1(inp))
print("Answer #2: ", part2(inp))
