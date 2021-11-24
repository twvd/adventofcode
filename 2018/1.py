import itertools

with open('1.txt') as f:
    input = [int(x) for x in f.readlines()]

print("Answer #1: %d" % sum(input))

freq = 0
freqs = set()
for i in itertools.cycle(input):
    freq += i
    if freq in freqs:
        print("Answer #2: %d" % freq)
        break
    freqs.add(freq)
