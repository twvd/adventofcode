import knothash
import itertools
import sys

ALLOCATED = -1
UNALLOCATED = 0
GRID_SIZE = 128

# Depth-first search to fill regions
def fillRegion(grid, regionNum, x, y):
    grid[x][y] = regionNum
    for dir in ((-1, 0), (0, -1), (1, 0), (0, 1)):
        if (x + dir[0]) >= GRID_SIZE or (y + dir[1]) >= GRID_SIZE or (x + dir[0]) < 0 or (y + dir[1]) < 0:
            continue
        if grid[x + dir[0]][y + dir[1]] == ALLOCATED:
            fillRegion(grid, regionNum, x + dir[0], y + dir[1])

def printGrid(grid):
    for y in range(GRID_SIZE):
        for x in range(GRID_SIZE):
            c = grid[x][y]
            if c == UNALLOCATED:
                c = '.'
            elif c == ALLOCATED:
                c = '#'
            else:
                c = str(c % 9)
            sys.stdout.write(c)
        print("")

inp = "xlqgujun"
answer1 = 0

# Generate the grid
grid = [[UNALLOCATED for i in range(GRID_SIZE)] for i in range(GRID_SIZE)]
for y in range(GRID_SIZE):
    key = inp + "-" + str(y)
    hash = knothash.knotHash2(key)

    for (idx, h), bit in itertools.product(enumerate(hash), range(8)):
        x = (idx * 8) + bit
        if h & (1 << (7 - bit)):
            grid[x][y] = ALLOCATED
            answer1 += 1

# Flood-fill the regions
regions = 0
for y, x in itertools.product(range(GRID_SIZE), range(GRID_SIZE)):
    if grid[x][y] != ALLOCATED:
        continue

    regions += 1
    fillRegion(grid, regions, x, y)

printGrid(grid)

print("Answer #1: %d" % answer1)
print("Answer #2: %d" % regions)