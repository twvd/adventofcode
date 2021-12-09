import itertools

with open('9.txt') as f:
    inp = {}
    w, h = 0, 0
    for y, l in enumerate(f.readlines()):
        for x, c in enumerate(l.strip()):
            inp[(x,y)] = int(c)
        w, h = x, y

adj_dir = ((-1, 0), (0, -1), (1, 0), (0, 1))

def adjacent(arr, x, y):
    for dx, dy in adj_dir:
        px, py = x + dx, y + dy
        if (px, py) in arr.keys():
            yield (px, py)

scanned = []
def scan_basin(arr, x, y):
    global scanned
    size = 0
    for bx, by in adjacent(arr, x, y):
        if arr[(bx, by)] != 9 and (bx, by) not in scanned:
            scanned += [(bx,by)]
            size += 1 + scan_basin(arr, bx, by)
    return size

risk = 0
basins = []           
for x, y in itertools.product(range(w+1), range(h+1)):
    adj_values = [inp[x] for x in adjacent(inp,x,y)]
    if min(adj_values) > inp[(x, y)]:
        risk += 1 + inp[(x,y)]
        basins += [scan_basin(inp, x, y)]

print("Answer #1:", risk)
ranked = list(reversed(sorted(basins)))
print("Answer #2:", ranked[0] * ranked[1] * ranked[2])
