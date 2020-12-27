import re
import copy
import numpy
from collections import defaultdict

criteria = {}
tickets = []

with open('16.txt') as f:
    inp = [l.strip() for l in f.readlines()]
    # Criteria
    for i in inp[:inp.index('')]:
        m = re.findall(r'(.*): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)', i)[0]
        criteria[m[0]] = ((int(m[1]), int(m[2])), (int(m[3]), int(m[4])))

    my_ticket = list(map(int, inp[inp.index('your ticket:')+1].split(',')))
        
    # Nearby tickets
    tickets = [list(map(int, i.split(','))) \
               for i in inp[inp.index('nearby tickets:')+1:]]
    
invalid = []
invalid_tickets = []
for t in tickets:
    for v in t:
        for c in criteria.values():
            if ((v >= c[0][0] and v <= c[0][1]) or \
                    (v >= c[1][0] and v <= c[1][1])):
                break
        else:
            invalid.append(v)
            invalid_tickets.append(t)

print("Answer #1: ", sum(invalid))

valid_tickets = [t for t in tickets if t not in invalid_tickets]
valid_tickets.append(my_ticket)

# First list all criteria that are valid
found = defaultdict(lambda: list())
for k, c in criteria.items():
    for col in range(len(valid_tickets[0])):
        for t in valid_tickets:
            v = t[col]
            if not ((v >= c[0][0] and v <= c[0][1]) or \
                    (v >= c[1][0] and v <= c[1][1])):
                break
        else:
            found[k].append(col)
            
# Resolve criteria with multiple columns by
# looking at criteria that have only 1 match and
# iteratively removing those from the results
done = {}
changed = True
while changed:
    changed = False
    # Move entries with 1 out
    for k, v in copy.deepcopy(found).items():
        if len(v) == 1:
            done[k] = v[0]
            changed = True
    # Remove entries/values that are already done
    for k, v in done.items():
        if k in found.keys():
            found.pop(k)
            changed = True
        else:
            for i in found.values():
                if v in i:
                    i.remove(v)
                    changed = True

departure = [my_ticket[v] for k, v in done.items() if k.startswith('departure')]
assert len(departure) == 6
print("Answer #2: ", numpy.prod(departure))
