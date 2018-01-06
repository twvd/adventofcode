def reversePart(hash, start, end):
    part = []
    size = len(hash)
    for i in range(start, end):
        part.append(hash[i % size])
    reverse = list(reversed(part))
    x = 0
    for i in range(start, end):
        hash[i % size] = reverse[x]
        x += 1
    return hash

def knotHash1(steps):
    hash = [i for i in range(256)]
    index = 0
    skipsize = 0

    for step in steps:
        start = index
        end = start + step
        hash = reversePart(hash, start, end)    
        index = (index + step + skipsize) % len(hash)
        skipsize += 1
    return hash

def knotHashDense(hash):
    dense = list(range(16))
    for b in range(16):
        h = 0
        for i in range(16):
            h ^= hash[(b * 16) + i]
        dense[b] = h
    return dense

def knotHash2(str):
    hash = [i for i in range(256)]
    steps = [ord(c) for c in str] + [17, 31, 73, 47, 23]
    index = 0
    skipsize = 0

    for round in range(64):
        for step in steps:
            start = index
            end = start + step
            hash = reversePart(hash, start, end)    
            index = (index + step + skipsize) % len(hash)
            skipsize += 1
    return knotHashDense(hash)

def knotHash2Str(str):
    hash = knotHash2(str)
    return ''.join('{:02x}'.format(x) for x in hash)
