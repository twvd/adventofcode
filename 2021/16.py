import numpy
from binascii import unhexlify
from io import StringIO

with open('16.txt') as f:
    inp = f.read().strip()

def binstream(s):
    # Convert input as ASCII-hex to a stream of single bits
    binp = bin(int.from_bytes(unhexlify(s), byteorder='big'))[2:]
    padding = '0' * (len(inp) * 4 - len(binp))
    return StringIO(padding + binp)

def part1(s):
    version = int(s.read(3), 2)
    type = int(s.read(3), 2)
    
    yield version
    
    if type == 4: # Literal
        val, next = '', 1
        while next:
            next = int(s.read(1), 2)
            val += s.read(4)
        val = int(val, 2)
    else: # Operator
        lentype = int(s.read(1), 2)
        if lentype == 1:
            numsub = int(s.read(11), 2)
            
            for i in range(numsub):
                yield from part1(s)
        else:
            sublen = int(s.read(15), 2)   
            subend = s.tell() + sublen
            while s.tell() < subend:
                yield from part1(s)

def part2(s):
    version = int(s.read(3), 2)
    type = int(s.read(3), 2)
    
    if type == 4: # Literal
        val, next = '', 1
        while next:
            next = int(s.read(1), 2)
            val += s.read(4)
        val = int(val, 2)
        yield val
    else: # Operator
        lentype = int(s.read(1), 2)
        vals = []
        if lentype == 1:
            numsub = int(s.read(11), 2)
            for i in range(numsub):
                vals += part2(s)
        else:
            sublen = int(s.read(15), 2)    
            subend = s.tell() + sublen
            vals = []
            while s.tell() < subend:
                vals += part2(s)
        if type == 0:
            yield sum(vals)
        elif type == 1:
            yield numpy.prod(vals)
        elif type == 2:
            yield min(vals)
        elif type == 3:
            yield max(vals)
        elif type == 5:
            assert len(vals) == 2
            yield 1 if vals[0] > vals[1] else 0
        elif type == 6:
            assert len(vals) == 2
            yield 1 if vals[0] < vals[1] else 0
        elif type == 7:
            assert len(vals) == 2
            yield 1 if vals[0] == vals[1] else 0
        else:
            print(f"Unknown type {type}")

print("Answer #1:", sum(part1(binstream(inp))))
print("Answer #2:", next(part2(binstream(inp))))
