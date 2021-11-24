with open('5.txt') as f:
    inp = f.readlines()

inp = [int(x.strip()) for x in inp]

ptr = 0
steps = 0

print("Instructions: %d" % len(inp))

while ptr >= 0 and ptr < len(inp):
    oldptr = ptr
    ptr = ptr + inp[ptr]
    inp[oldptr] += 1
    steps += 1

print("Answer 1: %d (end: %d)" % (steps, ptr))