import itertools

with open('4.txt') as f:
    inp = f.readlines()

answer1 = 0
answer2 = 0

for line in inp:
    words = line.strip().split(' ')

    # First policy
    uniquewords = len(set(words))
    if uniquewords == len(words):
        answer1 += 1
    else:
        continue

    # Second policy
    anagrams = []            
    for word in words:
        wanagrams = set(["".join(ana) for ana in itertools.permutations(word)])
        wanagrams.remove(word)
        anagrams += wanagrams
    
    found = False
    for word in words:
        if word in anagrams:
            found = True
            break

    if not found:
        answer2 += 1

print("Input lines: %d" % len(inp))
print("Answer 1: %d" % answer1)
print("Answer 2: %d" % answer2)