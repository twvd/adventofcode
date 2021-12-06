import itertools
from collections import deque

fish = deque(itertools.repeat(0, 9))

with open('6.txt') as f:
    for i in map(int, f.readline().strip().split(',')):
        fish[i] += 1

for day in range(256):
    if day == 80:
        print("Answer #1:", sum(fish))
    fish.rotate(-1)
    fish[6] += fish[8]

print("Answer #2:", sum(fish))
