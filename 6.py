with open('6.txt') as f:
    buckets = tuple(map(int, f.readline().strip().split('\t')))

seen = set()

steps = 0
lookForA2 = None

while True:
    steps += 1

    idx = buckets.index(max(buckets))
    spread = buckets[idx]

    newb = list(buckets)
    newb[idx] = 0

    while spread > 0:
        idx = (idx + 1) % len(buckets)
        newb[idx] += 1
        spread -= 1

    buckets = tuple(newb)

    if lookForA2 is None:
        if buckets in seen:
            print("Answer #1: %d" % steps)
            lookForA2 = buckets
            steps = 0
        seen.add(buckets)
    else:
        if buckets == lookForA2:
            print("Answer #2: %d" % steps)
            break