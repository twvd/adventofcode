class Maze:
    DIR_UP = 0
    DIR_RIGHT = 1
    DIR_DOWN = 2
    DIR_LEFT = 3

    def __init__(self, mapstr):
        self.map = mapstr

    def findStart(self):
        for x, c in enumerate(self.map[0]):
            if c == '|':
                return x, 0

    def isValidPosition(self, pos):
        return self.getAt(pos) != ' '

    def getAt(self, pos):
        return self.map[pos[1]][pos[0]]

    def findExit(self):
        collection = ""
        steps = 0
        pos = (self.findStart())
        direction = self.DIR_DOWN
        
        while pos != None:
            c = self.getAt(pos)
            if c.isalpha():
                collection += c

            pos, direction = self.findNextStep(pos, direction)
            steps += 1

        print("Answer #1: %s" % collection)
        print("Answer #2: %d" % steps)

    def calcPosDirection(self, pos, direction):
        if direction == self.DIR_RIGHT:
            return (pos[0] + 1, pos[1])
        elif direction == self.DIR_DOWN:
            return (pos[0], pos[1] + 1)
        elif direction == self.DIR_LEFT:
            return (pos[0] - 1, pos[1])
        elif direction == self.DIR_UP:
            return (pos[0], pos[1] - 1)

    def getValidTurns(self, d1):
        if d1 == self.DIR_DOWN or d1 == self.DIR_UP:
            return (self.DIR_LEFT, self.DIR_RIGHT)
        if d1 == self.DIR_RIGHT or d1 == self.DIR_LEFT:
            return (self.DIR_UP, self.DIR_DOWN)
        return False

    def findNextStep(self, pos, direction):
        newpos = self.calcPosDirection(pos, direction)

        if self.isValidPosition(newpos):
            # Can move on in current direction
            return newpos, direction

        # Try next directions
        for d in self.getValidTurns(direction):
            newpos = self.calcPosDirection(pos, d)
            if self.isValidPosition(newpos):
                return newpos, d

        # Stuck
        return None, direction

with open('19.txt') as f:
    inp = f.readlines()

maze = Maze(inp)
maze.findExit()