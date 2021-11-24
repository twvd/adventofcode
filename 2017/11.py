class Cube:
    def __init__(self, x = 0, y = 0, z = 0):
        self.x = x
        self.y = y
        self.z = z

    def add(self, x, y, z):
        self.x += x
        self.y += y
        self.z += z

    def addCube(self, c):
        self.add(c.x, c.y, c.z)

    def distanceFrom(self, other):
        return (abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)) / 2

with open('11.txt') as f:
    steps = f.readline().strip().split(',')

p = Cube()
directions = {
    'n':  Cube( 1,  0, -1),
    'ne': Cube( 1, -1,  0),
    'se': Cube( 0, -1,  1),
    's':  Cube(-1,  0,  1),
    'sw': Cube(-1,  1,  0),
    'nw': Cube( 0,  1, -1)
}

answer2 = 0
for step in steps:
    p.addCube(directions[step])
    answer2 = max(answer2, p.distanceFrom(Cube()))

print("Answer #1: %d" % p.distanceFrom(Cube()))
print("Answer #2: %d" % answer2)