import re

with open('2.txt') as f:
    inp = [(c, int(n)) for c, n in re.findall(r'(f|u|d).+ ([0-9]+)', f.read())]

p = {z: sum(n for i, n in inp if i == z) for z in set(j for j, _ in inp)}
print("Answer #1:", p['f'] * (p['d'] - p['u']))

aim, d = 0, 0
for c, n in inp:
    if c == 'f':
        d += aim * n
    else:
        aim += n if c == 'd' else -n

print("Answer #2:", p['f'] * d)
