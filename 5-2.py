with open('5.txt') as f:
    inp = f.readlines()

inp = [int(x.strip()) for x in inp]

ptr = 0
steps = 0
inplen = len(inp)

print("Instructions: %d" % inplen)

while ptr >= 0 and ptr < inplen:
    oldptr = ptr
    ptr = ptr + inp[ptr]
    if (inp[oldptr] >= 3):
        inp[oldptr] -= 1
    else:
        inp[oldptr] += 1
    steps += 1

print("Answer 2: %d (end: %d)" % (steps, ptr))