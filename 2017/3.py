import sys

def distance(p1, p2):
    x = abs(p1[0] - p2[0])
    y = abs(p1[1] - p2[1])
    return x + y

def getNextSpiralSpot(spiral, x, y, direction):
    if direction == 0: # Down
        if spiral[x + 1][y + 1] == 0:
            direction = 1
        y += 1
    elif direction == 1: # Right
        if spiral[x + 1][y - 1] == 0:
            direction = 2
        x += 1
    elif direction == 2: # Up
        y -= 1
        if spiral[x - 1][y] == 0:
            direction = 3
    elif direction == 3: # Left            
        x -= 1
        if spiral[x][y + 1] == 0:
            direction = 0
    return x, y, direction

def calcAnswer1(target, size):
    spiral = [[0]*size for i in range(size)]
    x = int(size / 2)
    y = int(size / 2)
    direction = 1
    for n in range(1, target + 1):
        spiral[x][y] = n
        if n == target:
            return x, y
        x, y, direction = getNextSpiralSpot(spiral, x, y, direction)

def calcAnswer2(target, size):
    spiral = [[0]*size for i in range(size)]
    x = int(size / 2)
    y = int(size / 2)
    direction = 1
    for n in range(1, target + 1):
        if n == 1:
            spiral[x][y] = n
        else:
            sum = spiral[x - 1][y - 1] \
                + spiral[x][y - 1] \
                + spiral[x - 1][y] \
                + spiral[x - 1][y + 1] \
                + spiral[x + 1][y] \
                + spiral[x + 1][y - 1] \
                + spiral[x][y + 1] \
                + spiral[x + 1][y + 1]
            if sum > target:
                return sum
            spiral[x][y] = sum
        x, y, direction = getNextSpiralSpot(spiral, x, y, direction)

def printSpiral(spiral):
    for y in range(0, len(spiral)):
        for x in range(0, len(spiral)):
            sys.stdout.write("%03d " % spiral[x][y])
        print("")

size = 5000
inp = 361527
x, y = calcAnswer1(inp, size)
print("Answer 1: %d" % distance((int(size / 2), int(size / 2)), (x, y)))
print("Answer 2: %d" % calcAnswer2(inp, size))
