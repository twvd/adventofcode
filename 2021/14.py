import re
from collections import defaultdict, Counter

with open('14.txt') as f:
    inp = f.readline().strip()
    f.readline()
    rules = {s[0]: s[1] for s in re.findall(r'([A-Z]+) -> ([A-Z])', f.read())}

def run(steps, start, rules):
    pairs = defaultdict(lambda: 0)
    letters = defaultdict(lambda: 0)
    for i, c in enumerate(start):
        b = inp[i:i+2]
        pairs[b] += 1
        letters[c] += 1

    for i in range(steps):
        newpairs = defaultdict(lambda: 0)
        for k, v in pairs.items():
            if k not in rules.keys():
                continue
            letters[rules[k]] += v
            newpairs[rules[k] + k[1]] += v
            newpairs[k[0] + rules[k]] += v
        pairs = newpairs

    c = Counter(letters).most_common()
    return c[0][1] - c[-1][1]
    
print("Answer #1:", run(10, inp, rules))
print("Answer #2:", run(40, inp, rules))
