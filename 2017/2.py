import itertools

with open('2.txt') as f:
    inp = f.readlines()

answer1 = 0
answer2 = 0

for row in inp:
    cols = list(map(int, row.strip().split('\t')))
    answer1 += abs(min(cols) - max(cols))

    for a, b in itertools.product(cols, cols):
        if a != b and a % b == 0:
            answer2 += a / b
            break

print("Answer 1: %d" % answer1)
print("Answer 2: %d" % answer2)
