from collections import Counter

with open('3.txt') as f:
    inp = [l.strip() for l in f.readlines()]

def part2(lst, common, default):  
    for i in range(len(lst[0])):
        c = Counter(list(zip(*lst))[i]).most_common()
        d = default if c[0][1] == c[-1][1] else c[common][0]
        lst = [x for x in lst if x[i] == d]
        if len(lst) == 1:
            break
    return int(lst[0], 2)

gamma = int(''.join(Counter(x).most_common()[0][0] for x in list(zip(*inp))), 2)
epsilon = gamma ^ ((1 << len(inp[0])) - 1)

print("Answer #1:", epsilon * gamma)
print("Answer #2:", part2(inp[:], 0, '1') * part2(inp[:], -1, '0'))
