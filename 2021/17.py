import itertools
import re

def launch(vx, vy, maxsteps=250):
    x, y = 0, 0

    for _ in range(maxsteps):
        x, y = x + vx, y + vy
        if vx > 0:
            vx -= 1
        elif vx < 0:
            vx += 1
        vy -= 1
        yield x, y

def find_trajectory(t0, t1, limit=200):
    for vx, vy in itertools.product(range(limit), range(-limit, limit)):
        for x, y in launch(vx, vy):
            if x >= t0[0] and y >= t0[1] and x <= t1[0] and y <= t1[1]:
                yield vx, vy
            if x >= t1[0] and y >= t1[1]:
                # Overshot..
                break

def find_highest_y(t0, t1):
    for vx, vy in find_trajectory(t0, t1):
        xy = max(list(zip(*launch(vx, vy)))[1])
        yield xy

with open('17.txt') as f:
    t0x, t1x, t0y, t1y = map(int, re.findall(r'x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)', f.read())[0])
    t0, t1 = (t0x, t0y), (t1x, t1y)

print("Answer #1:", max(find_highest_y(t0, t1)))
print("Answer #2:", len(set(find_trajectory(t0, t1))))