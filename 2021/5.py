import itertools
import re
from collections import defaultdict

def line(omap, x0, y0, x1, y1, part2=False):
    if not part2 and (x0 == x1 or y0 == y1):
        return

    # https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    dx, dy = abs(x1 - x0), -abs(y1 - y0)
    signx, signy = 1 if x0 < x1 else -1, 1 if y0 < y1 else -1
    err = dx + dy
    while True:
        omap[(x0,y0)] += 1
        if x0 == x1 and y0 == y1:
            break
        if 2 * err >= dy:
            err += dy
            x0 += signx
        if 2 * err <= dx:
            err += dx
            y0 += signy

def run(inp, part2=False):
    omap = defaultdict(lambda: 0)
    for i in inp:
        line(omap, *map(int, i), part2)
    return len([p for p in omap.values() if p > 1])

with open('5.txt') as f:
    inp = re.findall(r'(\d+),(\d+) -> (\d+),(\d+)', f.read())

print("Answer #1:", run(inp))
print("Answer #2:", run(inp, True))