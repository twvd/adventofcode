with open('6.txt') as f:
    buckets = list(map(int, f.readline().strip().split('\t')))

seen = set()

steps = 0
lookForA2 = None

while True:
    steps += 1

    idx = buckets.index(max(buckets))
    spread = buckets[idx]

    buckets[idx] = 0

    while spread > 0:
        idx = (idx + 1) % len(buckets)
        buckets[idx] += 1
        spread -= 1

    tbuckets = tuple(buckets)

    if lookForA2 is None:
        if tbuckets in seen:
            print("Answer #1: %d" % steps)
            lookForA2 = tbuckets
            steps = 0
        seen.add(tbuckets)
    else:
        if tbuckets == lookForA2:
            print("Answer #2: %d" % steps)
            break