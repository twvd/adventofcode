import itertools
import re

def deterministic_dice_rolls(rolls=3):
    it = itertools.cycle(range(1, 101))
    while True:
        yield sum(itertools.islice(it, 3))

def inc_player_pos(pos, inc):
    return ((pos - 1) + inc) % 10 + 1

def runp1(p1start, p2start):
    dice = deterministic_dice_rolls()
    pos = [p1start, p2start] 
    scores = [0, 0]

    for turn, p in enumerate(itertools.cycle(range(2)), 1):
        pos[p] = inc_player_pos(pos[p], next(dice))
        scores[p] += pos[p] 
        if scores[p] >= 1000:
            return scores[p ^ 1] * turn * 3

with open('21.txt') as f:
    starts = list(map(int, re.findall(r': (\d+)', f.read())))

print("Answer #1: ", runp1(*starts))
