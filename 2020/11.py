import copy
import itertools

with open('11.txt') as f:
    inp = [list(l.strip()) for l in f.readlines()]

OCCUPIED, EMPTY, NOSEAT = '#', 'L', '.'
    
width, height = len(inp[0]), len(inp)
    
def get_occupied(state, x, y, part2 = False):
    occupied = 0
    #print("occ ", x, y)
    for rx, ry in itertools.product(range(-1, 2), range(-1, 2)):
        if (rx, ry) == (0, 0):
            continue
        sx, sy = x, y
        valid = True
        while True:
            sx, sy = sx + rx, sy + ry
            if sx < 0 or sy < 0 or sx >= width or sy >= height:
                valid = False
                break
            if state[sy][sx] != NOSEAT or not part2:
                break
        if not valid:
            continue
        if (state[sy][sx] == OCCUPIED):
            occupied += 1
    return occupied
    
def part_step(state, part2 = False):
    newstate = copy.deepcopy(state)
    changes = 0
    for x, y in itertools.product(range(width), range(height)):
        occ = get_occupied(state, x, y, part2)
        if state[y][x] == EMPTY and occ == 0:
            newstate[y][x] = OCCUPIED
            changes += 1
        elif state[y][x] == OCCUPIED:
            if (not part2 and occ >= 4) or \
                (part2 and occ >= 5):
                newstate[y][x] = EMPTY
                changes += 1
    return newstate, changes

def part(state, part2 = False):
    for step in itertools.count(1):
        newstate, changes = part_step(state, part2)
        if changes == 0:
            break
        state = newstate

    return sum([r.count(OCCUPIED) for r in state])
        
print("Answer #1:", part(inp))
print("Answer #2:", part(inp, True))
