import networkx

graph = networkx.Graph()

with open('12.txt') as f:
    for l in f.readlines():
        l = l.strip()
        n, leafs = l.split(' <-> ')
        graph.add_node(int(n))
        for leaf in leafs.split(', '):
            graph.add_edge(int(n), int(leaf))

print("Answer #1: %d" % len(networkx.node_connected_component(graph, 0)))
print("Answer #2: %d" % networkx.number_connected_components(graph))