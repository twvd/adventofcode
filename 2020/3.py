import itertools

with open('3.txt') as f:
    inpmap = [l.strip() for l in f.readlines()]

def mapxy(x, y):
    return inpmap[y][x % len(inpmap[0])]

def traverse(right, down):
    trees = 0
    x = 0
    for y in range(down, len(inpmap), down):
        x += right
        if mapxy(x, y) == '#':
            trees += 1
    return trees

print("Answer #1: ", traverse(3, 1))

step2 = ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2))
ans2 = 1
for s in step2:
    ans2 *= traverse(s[0], s[1])
print("Answer #2: ", ans2)
