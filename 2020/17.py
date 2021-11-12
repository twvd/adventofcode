import copy
import itertools
from collections import defaultdict

EMPTY = '.'
FILLED = '#'

class Field(defaultdict):
    def __init__(self, *args):
        if args:
            super(Field, self).__init__(*args)
        else:
            super(Field, self).__init__(lambda: False)

    def xmin(self):
        return min(x for x, _, _, _ in self.keys())
    def xmax(self):
        return max(x for x, _, _, _ in self.keys())
    def ymin(self):
        return min(y for _, y, _, _ in self.keys())
    def ymax(self):
        return max(y for _, y, _, _ in self.keys())
    def zmin(self):
        return min(z for _, _, z, _ in self.keys())
    def zmax(self):
        return max(z for _, _, z, _ in self.keys())
    def wmin(self):
        return min(w for _, _, _, w in self.keys())
    def wmax(self):
        return max(w for _, _, _, w in self.keys())
    def ix(self, e=0):
        return range(self.xmin() - e, self.xmax() + e + 1)
    def iy(self, e=0):
        return range(self.ymin() - e, self.ymax() + e + 1)
    def iz(self, e=0):
        return range(self.zmin() - e, self.zmax() + e + 1)
    def iw(self, e=0):
        return range(self.wmin() - e, self.wmax() + e + 1)
    def ixyz(self, e=0):
        return itertools.product(self.ix(e), self.iy(e), self.iz(e))
    def ixyzw(self, e=0):
        return itertools.product(self.ix(e), self.iy(e), self.iz(e), self.iw(e))
    def count(self):
        return len([v for v in self.values() if v])

field = Field()

with open('17.txt') as f:
    for y, l in enumerate(f.readlines()):
        for x, c in enumerate(l.strip()):
            if c == FILLED:
                field[(x,y,0,0)] = True

def step(field, parttwo=False):
    surr = (-1, 0, 1)
    newfield = copy.deepcopy(field)

    for x, y, z, w in newfield.ixyzw(1):
        neighbours = 0
        if not parttwo and w != 0:
            continue
        for sx, sy, sz, sw in itertools.product(surr, surr, surr, surr):
            ix, iy, iz, iw = x + sx, y + sy, z + sz, w + sw
            if (ix, iy, iz, iw) == (x, y, z, w) or (not parttwo and iw != 0):
                continue
            if field[(ix, iy, iz, iw)]:
                neighbours += 1
        if field[(x, y, z, w)] and neighbours in (2, 3):
            newfield[(x, y, z, w)] = True
        elif not field[(x, y, z, w)] and neighbours == 3:
            newfield[(x, y, z, w)] = True
        else:
            newfield[(x, y, z, w)] = False
    return newfield

p1field = copy.deepcopy(field)

for c in range(6):
    p1field = step(p1field, False)

print("Answer #1: ", p1field.count())

p2field = copy.deepcopy(field)
for c in range(6):
    p2field = step(p2field, True)

print("Answer #2: ", p2field.count())
