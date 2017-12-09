import re

class Program:
    def __init__(self, name, weight):
        self.name = name
        self.children = {}
        self.weight = weight

    def printTree(self, maxdepth, depth = 0):
        if (depth > maxdepth):
            return
        print("%s %s(%d:%d:%d)" % (" "*(depth*12), self.name, self.weight, self.getWeightOfChildren(), self.getTotalWeight()))
        for child in self.children.values():
            child.printTree(maxdepth, depth + 1)

    def getWeightOfChildren(self):
        return sum((c.weight + c.getWeightOfChildren()) for c in self.children.values())

    def getTotalWeight(self):
        return self.weight + self.getWeightOfChildren()

    def getUniqueWeights(self):
        return {c.getTotalWeight() for c in self.children.values()}

    def childrenBalanced(self):        
        return len(self.getUniqueWeights()) == 1

    def findImbalance(self, depth = 0):
        if self.childrenBalanced():
            # Everything underneath is balanced, so skip this branch
            return True

        # Recurse into children
        for c in self.children.values():
            if not c.findImbalance(depth + 1):
                return False

        # Everything above the children is balanced, so the offender is one of the
        # childrens own weight. Find the heaviest one to adjust.
        weights = self.getUniqueWeights()
        adjustment = max(weights) - min(weights)
        for c in self.children.values():
            if max(weights) == c.getTotalWeight():
                # This one is too heavy
                print("Answer #2: %d - offender: %s (%d too heavy)" % ((c.weight - adjustment), c.name, adjustment))
                return False

def findRoot(prog):
    for prog2 in programs.values():
        if prog in prog2.children.values():
            return findRoot(prog2)
    return prog

programs = {}
inputMatch = re.compile('([a-z]*) \(([0-9]*)\)( -> (.*))?')

with open('7.txt') as f:
    # Create all programs first
    for line in f.readlines():
        pName, pWeight, _, pChildren = inputMatch.findall(line)[0]
        programs[pName] = Program(pName, int(pWeight))
    
    f.seek(0)
    # Link children now
    for line in f.readlines():
        pName, pWeight, _, pChildren = inputMatch.findall(line)[0]
        if pChildren == '':
            continue

        for child in map(str.strip, pChildren.split(',')):
            programs[pName].children[child] = programs[child]

root = findRoot(programs[next(iter(programs))])
print("Answer #1: %s" % root.name)
root.findImbalance()