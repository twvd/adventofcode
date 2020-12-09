import re

with open('7.txt') as f:
    inp = f.read()

# Parse input into nested dictionaries
# Key = bag color, value = amount of bags
rules = {}

for k, v in re.findall(r'(.+) contain (.+)', inp):
    # Get bag color minus 'bag'
    k = ' '.join(k.split(' ')[:2])
    links = {}
    for l in v.split(', '):
        l = l.split(' ')
        if l[0] == 'no': # 'no other bags'
            continue
        links[' '.join(l[1:3])] = int(l[0])
    rules[k] = links
    
def part1(n):
    c = [k for k, v in rules.items() if n in v.keys()]
    a = set(c)
    for v in c:
        a |= part1(v)
    return a

def part2(n, total = 0):
    total += sum(rules[n].values())
    for k, v in rules[n].items():
        total += (part2(k) * v)
    return total
    
print("Answer #1: ", len(part1('shiny gold')))
print("Answer #2: ", part2('shiny gold'))
