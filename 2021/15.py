import networkx

def wrapinc(i, inc):
    ni = (i + inc) % 10
    if i > ni:
        ni += 1
    return ni

def fieldwh(field):
    maxx, maxy = map(max, zip(*field.keys()))
    return maxx + 1, maxy + 1

def path(field):
    graph = networkx.DiGraph()
    width, height = fieldwh(field)
    target1 = (width - 1, height - 1)

    for (x, y), i in field.items():
        if x + 1 < width:
            graph.add_edge((x, y), (x + 1, y), weight=field[x + 1, y])
        graph.add_edge((x + 1, y), (x, y), weight=i)
        if y + 1 < height:
            graph.add_edge((x, y), (x, y + 1), weight=field[x, y + 1])
        graph.add_edge((x, y + 1), (x, y), weight=i)

    return sum([field[x, y] for x, y in networkx.shortest_path(graph, source=(0, 0), target=target1, weight='weight')]) - field[0, 0]

def expand(field):
    width, height = fieldwh(field)

    expanded = field.copy()
    # Expand horizontally
    for (x, y), i in inp.items():
        for e in range(1, 5):
            expanded[width * e + x, y] = wrapinc(i, e)
    # Expand vertically
    for (x, y), i in expanded.copy().items():
        for e in range(1, 5):
            expanded[x, height * e + y] = wrapinc(i, e)
    return expanded


with open('15.txt') as f:
    inp = {(x, y): int(c) for y, l in enumerate(f) for x, c in enumerate(l.strip())}

print("Answer #1:", path(inp))
print("Answer #2:", path(expand(inp)))