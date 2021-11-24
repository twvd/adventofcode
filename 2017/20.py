import copy
import itertools
import numpy
import re
from tqdm import tqdm

def distance(pos1, pos2):
    x = abs(pos1[0] - pos2[0])
    y = abs(pos1[1] - pos2[1])
    z = abs(pos1[2] - pos2[2])
    return x + y + z

class Particle:
    def __init__(self, position, velocity, acceleration):
        self.position = position
        self.velocity = velocity
        self.acceleration = acceleration

    def move(self):
        self.velocity = numpy.add(self.velocity, self.acceleration)
        self.position = numpy.add(self.position, self.velocity)

matchInput = re.compile('p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>')
particles = []

with open('20.txt') as f:
    for line in f.readlines():
        mi = matchInput.findall(line.strip())[0]
        mi = [i for i in map(int, mi)]
        p = Particle((mi[0], mi[1], mi[2]), (mi[3], mi[4], mi[5]), (mi[6], mi[7], mi[8]))
        particles.append(p)

closest = []
# Get a copy for puzzle 1
particles1 = copy.deepcopy(particles)

for i in tqdm(range(2000)):
    dists = []
    for p in particles1:
        p.move()
        dists.append(distance((0,0,0), p.position))
    
    closest.append(dists.index(min(dists)))
print("Answer #1: %d" % numpy.bincount(closest).argmax())

for i in tqdm(range(100)):
    for p in particles:
        p.move()

    ps = particles.copy()
    for p1, p2 in itertools.product(ps, ps):
        if p1 == p2:
            continue
        if p1.position[0] == p2.position[0] and p1.position[1] == p2.position[1] and p1.position[2] == p2.position[2]:                
            try:
                particles.remove(p1)
                particles.remove(p2)
            except ValueError:
                pass

print("Answer #2: %d" % len(particles))