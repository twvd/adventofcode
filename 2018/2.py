import itertools

with open('2.txt') as f:
    input = [s.strip() for s in f.readlines()]

# Answer #1
two = 0
three = 0

for i in input:
    # Build a dictionary to count letters
    letters = {}
    for c in i:
        if c not in letters.keys():
            letters[c] = 1
        else:
            letters[c] += 1

    # Check dict for 2 and 3 for final checksum
    if 2 in letters.values():
        two += 1
    if 3 in letters.values():
        three += 1

# Calculate final checksum
answer1 = (two * three)
print("Answer #1: %d" % answer1)

# Answer #2 - compare all inputs to check differences
for a, b in itertools.product(input, input):
    if a == b:
        continue

    diff = 0

    # Count differences
    for ac, bc in zip(a, b):
        if ac != bc: diff += 1

    if diff == 1:
        # Build the answer out of the equal characters in each string
        answer2 = ''.join([ac for ac, bc in zip(a, b) if ac == bc])
        print("Answer #2: %s" % answer2)
        break