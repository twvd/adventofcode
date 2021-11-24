with open('1.txt') as f:
    inp = f.readline().strip()

inplen = len(inp)

# Alternative, one-line approach:
# answer1 = sum([0, int(c)][c == inp[int((idx + 1) % inplen)]] for idx, c in enumerate(inp))
# answer2 = sum([0, int(c)][c == inp[int((idx + (inplen / 2)) % inplen)]] for idx, c in enumerate(inp))

# Simple approach:
answer1 = 0
answer2 = 0

for idx, c in enumerate(inp):
    a1idx = int((idx + 1) % inplen)
    a2idx = int((idx + (inplen / 2)) % inplen)
    if c == inp[a1idx]:
        answer1 += int(c)
    if c == inp[a2idx]:
        answer2 += int(c)

print("Answer 1: %d" % answer1)
print("Answer 2: %d" % answer2)
