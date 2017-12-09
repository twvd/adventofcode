import re

with open('9.txt') as f:
    inp = f.readline().strip()

# Remove all not'ted-characters
inp = re.sub('\!.', '', inp)

# Count all garbage
garbageExp = re.compile('<[^>]*>')
answer2 = 0
for match in garbageExp.findall(inp):
    answer2 += (len(match) - 2)

# Remove all garbage
inp = garbageExp.sub('', inp)

# Score the groups
depth = 0
answer1 = 0
for i in range(len(inp)):
    if (inp[i] == '{'):
        depth += 1
        answer1 += depth
    elif (inp[i] == '}'):
        depth -= 1

print("Answer #1: %d" % answer1)
print("Answer #2: %d" % answer2)