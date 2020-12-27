with open('14.txt') as f:
    inp = [l.strip() for l in f.readlines()]

BITS = 36
    
def parse_mask(m):
    one, zero, floating = 0, 0, 0
    for i in range(BITS):
        if m[BITS-1-i] == '1':
            one |= (1 << i)
        elif m[BITS-1-i] == '0':
            zero |= (1 << i)
        elif m[BITS-1-i] == 'X':
            floating |= (1 << i)
    return zero, one, floating

def part1(inp):
    mem = {}
    mask_zero, mask_one = 0, 0
    for o, v in [l.split(' = ') for l in inp]:
        if o == 'mask':
            mask_zero, mask_one, _ = parse_mask(v)
        else: # mem[x]
            addr = int(o[4:-1])
            mem[addr] = (int(v) | mask_one) & ~mask_zero
    return sum(mem.values())

def part2(inp):
    mem = {}
    mask_one, mask_floating = 0, 0
    for o, v in [l.split(' = ') for l in inp]:
        if o == 'mask':
            _, mask_one, mask_floating = parse_mask(v)
        else: # mem[x]
            addrs = [int(o[4:-1]) | mask_one]
            for i in [i for i in range(BITS) if mask_floating & (1 << i)]:
                for a in addrs[:]:
                    addrs.append(a | (1 << i))
                    addrs.append(a & ~(1 << i))
                    
            for addr in addrs:
                mem[addr] = int(v)
    
    return sum(mem.values())

print("Answer #1: ", part1(inp))
print("Answer #2: ", part2(inp))
