from collections import Counter

with open('8.txt') as f:
    inp = [[[sorted(s) for s in p.strip().split(' ')] for p in l.strip().split('|')] for l in f.readlines()]

# Part 1
outputs = sum(sum(inp, [])[1::2], [])
c = Counter([len(x) for x in outputs])
# 1 = 2 segments, 4 = 4 segments, 7 = 3 segments, 8 = 7 segments
print(f"Answer #1: {c[2] + c[4] + c[3] + c[7]}")

# Part 2
"""
   0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

Total occurrences of segments:
a = 8
b = 6
c = 8
d = 7
e = 4
f = 9
g = 7
 """
digits = ['abcefg', 'cf', 'acdeg', 'acdfg', 'bcdf', 'abdfg', 'abdefg', 'acf', 'abcdefg', 'abcdfg']
nums = []
for entry in inp:
    occurrences = Counter(sum(entry[0], []))

    # 2 (1) segment reveals C, F
    one = set([x for x in entry[0] if len(x) == 2][0])
    # 4 (4) segment reveals B, C, D, F
    four = set([x for x in entry[0] if len(x) == 4][0])
    # 7 (3) segment reveals A, C, F
    seven = set([x for x in entry[0] if len(x) == 3][0])
    # 8 (7) segment reveals A, B, C, D, E, F, G
    eight = set([x for x in entry[0] if len(x) == 7][0])

    # 7 adds A to 1, so can deduce segment A.
    seg_a, = seven - one
    # B is the only one occuring 6 times, so deduce that.
    seg_b, = [k for k, v in occurrences.items() if v == 6]
    # E is the only one occuring 4 times, so deduce that.
    seg_e, = [k for k, v in occurrences.items() if v == 4]
    # F is the only one occuring 9 times, so deduce that.
    seg_f, = [k for k, v in occurrences.items() if v == 9]
    
    # Now we can deduce the rest of the segments
    seg_c, = seven - set([seg_a, seg_f])
    seg_d, = four - set([seg_b, seg_c, seg_f])
    seg_g, = eight - set([seg_a, seg_b,  seg_c, seg_d, seg_e, seg_f])
    
    mapping = {
        seg_a: 'a',
        seg_b: 'b',
        seg_c: 'c',
        seg_d: 'd',
        seg_e: 'e',
        seg_f: 'f',
        seg_g: 'g'
    }
    translated = [''.join(sorted([mapping.get(c) for c in ''.join(e)])) for e in entry[1]]
    nums += [int(''.join([str(digits.index(d)) for d in translated]))]

print("Answer #2:", sum(nums))
