from collections import defaultdict

with open('12.txt') as f:
    paths = defaultdict(lambda: set())
    for a, b in [l.strip().split('-') for l in f]:
        if b != 'start':
            paths[a] = paths[a].union([b])
        if a != 'start':
            paths[b] = paths[b].union([a])

def search_paths(paths, lcvisits=1):
    search = [['start']]

    while len(search) > 0:
        c = search.pop()
        if c[-1] == 'end':
             yield c
        else:
            lcv = max(c.count(x) for x in c if x.islower())
            for p in paths[c[-1]]:
                if p.islower() and (p in c and lcv >= lcvisits):
                    continue
                search.append(c + [p])

print("Answer #1:",  len(list(search_paths(paths))))
print("Answer #2:",  len(list(search_paths(paths, 2))))
