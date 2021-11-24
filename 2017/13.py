def calcScanner(time, depth):
    scanner = time % ((depth - 1) * 2)
    if scanner >= depth:
        scanner = ((depth - 1) * 2) - scanner
    return scanner

def answer1():
    sev = 0
    for i in depths.keys():
        if calcScanner(i, depths[i]) == 0:
            sev += (i * depths[i])
    return sev

def answer2():
    delay = 0
    while True:
        stopped = False
        for i in depths.keys():
            if calcScanner(i + delay, depths[i]) == 0:
                delay += 1
                stopped = True
                break
        if not stopped:
            break
    return delay

depths = {}

with open('13.txt') as f:
    for l in f.readlines():
        layer, depth = l.strip().split(': ')
        depths[int(layer)] = int(depth)

print("Answer #1: %d" % answer1())
print("Answer #2: %d" % answer2())