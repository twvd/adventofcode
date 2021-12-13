import re
from collections import defaultdict

with open('13.txt') as f:
    l = [x.strip() for x in f.readlines()]
    coords = [map(int, s.split(',')) for s in l[:l.index('')]]
    folds = [(axis, int(n)) for axis, n in re.findall(r'fold along (.)=(\d+)', ''.join(l[l.index('')+1:]))]

def printpaper(paper):
    mx, my = map(max, zip(*paper.keys()))
    for y in range(my + 1):
        print(''.join('#' if paper[(x, y)] else ' ' for x in range(mx + 1)))

def foldy(paper, fold_y):
    for x, y in list(paper.keys()):
        if y < fold_y:
            continue
        if paper[(x, y)]:
            paper[(x, 2 * fold_y - y)] = paper[(x, y)]
        del paper[(x, y)]

def foldx(paper, fold_x):
    for x, y in list(paper.keys()):
        if x < fold_x:
            continue
        if paper[(x, y)]:
            paper[(2 * fold_x - x, y)] = paper[(x, y)]
        del paper[(x, y)]

paper = defaultdict(lambda: False, {(x, y): True for x, y in coords})
for axis, n in folds:
    if axis == 'x':
        foldx(paper, n)
    else:
        foldy(paper, n)
    print(f"Fold {axis} {n}: {len([x for x in paper.values() if x])} dots")
printpaper(paper)
