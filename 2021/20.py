from collections import defaultdict
from copy import copy
import itertools
import sys

with open('20.txt') as f:
    algorithm = [True if c == '#' else False for c in f.readline().strip()]
    f.readline()
    image = defaultdict(lambda: False, {(x, y): True if c == '#' else False
                        for y, l in enumerate(f)
                        for x, c in enumerate(l.strip())})

SQUARE = (
    (-1,-1),( 0,-1),( 1,-1),
    (-1, 0),( 0, 0),( 1, 0),
    (-1, 1),( 0, 1),( 1, 1)
)

def enhance_pixel(img, algo, x, y):
    bits = 0
    for bit, (sx, sy) in enumerate(SQUARE):
        nx, ny = x + sx, y + sy
        if img[(nx, ny)]:
            bits |= (1 << (8 - bit))
    return algo[bits]

def enhance(img, algo, default):
    newimg = img.copy()
    newimg.default_factory = lambda: default
    maxx, maxy = map(max, zip(*img.keys()))
    minx, miny = map(min, zip(*img.keys()))
    padding = len(SQUARE[0])

    for x, y in itertools.product(range(minx - padding, maxx + padding + 1),
                                  range(miny - padding, maxy + padding + 1)):
        newimg[(x, y)] = enhance_pixel(img, algo, x, y)

    return newimg

def run(times, image, algorithm):
    img = image.copy()
    for i in range(times):
        default = algorithm[0] if (i % 2) == 0 else algorithm[511]
        img = enhance(img, algorithm, default)
    return len([x for x in img.values() if x])

print("Answer #1:", run(2, image, algorithm))
print("Answer #2:", run(50, image, algorithm))

