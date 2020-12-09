import itertools

with open('9.txt') as f:
    inp = [int(l) for l in f.readlines()]

PREAMBLE_LEN = 25

def part1(inp):
    for p in range(PREAMBLE_LEN, len(inp)):
        window = inp[p - PREAMBLE_LEN:p]
        ok = False
        for a, b in itertools.product(window, window):
            if a + b == inp[p]:
                break
        else:
            return inp[p]

def part2(inp, search):            
    for p in range(0, len(inp)):
        for inc in range(2, len(inp) - p):
            window = inp[p:p+inc]
            test = sum(window)
            if test > search:
                break
            if test == search:
                return min(window) + max(window)

ans1 = part1(inp)
print("Answer #1: ", ans1)
print("Answer #2: ", part2(inp, ans1))
