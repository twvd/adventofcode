from collections import Counter, defaultdict
import numpy

with open('10.txt') as f:
    inp = sorted([int(l) for l in f.readlines()])

inp = [0] + inp + [max(inp) + 3]

diffs = Counter([inp[i + 1] - inp[i] for i in range(len(inp) - 1)])
print("Answer #1:", numpy.prod(list(diffs.values())))

acc = defaultdict(lambda: 0)
acc[0] = 1
for i in inp:
    for j in range(i - 3, i):
        acc[i] += acc[j]
print("Answer #2:", max(acc.values()))
