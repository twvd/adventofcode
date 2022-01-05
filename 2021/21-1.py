import itertools
import re

def wraprange(mn, mx, skip=0):
    it = itertools.cycle(range(mn, mx + 1))
    for _ in range(skip): next(it)
    yield from it

def deterministic_dice():
    yield from wraprange(1, 100)

def player_pos(start):
    yield from wraprange(1, 10, start)

def do_roll(edice, rolls=3):
    roll = list(zip(*itertools.islice(edice, rolls)))
    return sum(roll[1]), max(roll[0])

def inc_player_pos(pos, inc):
    return list(itertools.islice(pos, inc))[-1]

def runp1(p1start, p2start):
    dice = enumerate(deterministic_dice(), 1)
    pos = (player_pos(p1start), player_pos(p2start))
    scores = [0, 0]

    for p in itertools.cycle(range(2)):
        roll_inc, roll_count = do_roll(dice)
        scores[p] += inc_player_pos(pos[p], roll_inc)
        if scores[p] >= 1000:
            return scores[p ^ 1] * roll_count

with open('21.txt') as f:
    starts = list(map(int, re.findall(r': (\d+)', f.read())))

print("Answer #1: ", runp1(*starts))
