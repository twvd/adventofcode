import itertools

with open('11.txt') as f:
    inp = {(x, y): int(c)
                   for y, l in enumerate(f)
                   for x, c in enumerate(l.strip())}

def adjacent(field, x, y):
    for dx, dy in set(itertools.product(*itertools.repeat((-1, 0, 1), 2))) - set((0,0)):
        if (x + dx, y + dy) in field.keys():
            yield x + dx, y + dy

def step(field):
    flashed = []
    done = []
    # Increment full field
    for k in field.keys():
        field[k] = (field[k] + 1) % 10
        if field[k] == 0:
            flashed.append(k)
    # Recurse the splash-effect from the flash(es)
    while len(flashed) > 0:
        flash = flashed.pop()
        done.append(flash)
        for k in set(adjacent(field, *flash)) - set(flashed) - set(done):
            field[k] = (field[k] + 1) % 10
            if field[k] == 0:
                flashed.append(k)
    return len(done)

answer1 = 0
for i in itertools.count(1):
    s = step(inp)
    answer1 += s
    if i == 100:
        print("Answer #1:", answer1)
    if s == len(inp):
        print("Answer #2:", i)
        break
