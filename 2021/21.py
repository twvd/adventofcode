import functools
import itertools
import re
import time
from collections import Counter

NUM_ROLLS = 3
NUM_PLAYERS = 2
NUM_POSITIONS = 10

# For the quantum dice, lump all combinations of sides that yield the
# same position increment. We just multiply a win by the number of combinations.
quantum_dice_rolls = Counter(sum(r) for r in itertools.product(range(1, 4), repeat=NUM_ROLLS))

def deterministic_dice_rolls(rolls=NUM_ROLLS):
    it = itertools.cycle(range(1, 101))
    while True:
        yield sum(itertools.islice(it, NUM_ROLLS))

def inc_player_pos(pos, inc):
    return ((pos - 1) + inc) % NUM_POSITIONS + 1

def runp1(p1start, p2start):
    dice = deterministic_dice_rolls()
    pos = [p1start, p2start]
    scores = [0, 0]

    for turn, p in enumerate(itertools.cycle(range(NUM_PLAYERS)), 1):
        pos[p] = inc_player_pos(pos[p], next(dice))
        scores[p] += pos[p]
        if scores[p] >= 1000:
            return scores[p ^ 1] * turn * NUM_ROLLS

# Use caching to avoid recursing down the same path multiple times.
@functools.lru_cache(maxsize=None)
def p2turn(pos, pos_other, score=0, score_other=0):
    global quantum_dice_rolls

    wins = [0, 0]
    for rollsum, count in quantum_dice_rolls.items():
        posnew = inc_player_pos(pos, rollsum)
        scorenew = score + posnew
        if scorenew >= 21:
            # End of this path, active player won this turn.
            wins[0] += count
            continue
        
        # Other player takes a turn. Multiply wins to account for lumping
        # the dice combinations.
        for i, w in enumerate(reversed(p2turn(pos_other, posnew, score_other, scorenew))):
            wins[i] += w * count
    return wins


with open('21.txt') as f:
    starts = list(map(int, re.findall(r': (\d+)', f.read())))

print("Answer #1:", runp1(*starts))
p2t_start = time.time()
print("Answer #2:", max(p2turn(*starts)))
p2t = time.time() - p2t_start

print("-"*20)
print(f"Part 2 ran in {p2t:.4f} s")
print(p2turn.cache_info())
