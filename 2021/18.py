from ast import literal_eval
from collections.abc import Iterable
from math import ceil
import itertools

def incpair(v, d, i):
    return (v + i, d)

def lexplode(flatl):
    for idx, (v, d) in enumerate(flatl):
        if d > 4:
            # Left addition
            if idx > 0:
                flatl[idx-1] = incpair(*flatl[idx-1], v)
            # Right addition
            if idx + 2 < len(flatl):
                flatl[idx+2] = incpair(*flatl[idx+2], flatl[idx+1][0])
            del flatl[idx+1]
            flatl[idx] = (0, d - 1)
            return True
    return False
    
def lsplit(flatl):
    for idx, (v, d) in enumerate(flatl):
        if v >= 10:
            flatl[idx] = (v // 2, d + 1)
            flatl.insert(idx + 1, (ceil(v / 2), d + 1))
            return True
    return False
    
def lreduce(flatl):
    while True:
        if lexplode(flatl):
            continue
        if lsplit(flatl):
            continue
        break

def ladd(flatl, flatlnew):
    return [(v, d + 1) for v, d in flatl + flatlnew]
        
def addreduce(flatl1, flatl2):
    newl = ladd(flatl1, flatl2)
    lreduce(newl)
    return newl

def flatten(l, d=1):
    for el in l:
        if isinstance(el, Iterable):
            yield from flatten(el, d+1)
        else:
            yield (el, d)

def magnitude(flatl):
    while len(flatl) > 1:
        for i, ((v1, d1), (v2, d2)) in enumerate(zip(flatl, flatl[1:])):
            if d1 != d2:
                continue
            flatl[i] = (3 * v1 + 2 * v2, d1 - 1)
            del flatl[i+1]
            break
    return flatl[0][0]

with open('18.txt') as f:
    inp = [list(flatten(literal_eval(l.strip()))) for l in f]

state = inp[0]
for i in inp[1:]:
    state = addreduce(state, i)
    
print("Answer #1:", magnitude(state))
print("Answer #2:", max(magnitude(addreduce(*candidates)) for candidates in itertools.permutations(inp, 2)))
