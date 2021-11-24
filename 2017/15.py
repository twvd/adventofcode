from tqdm import tqdm

class Generator:
    def __init__(self, seed, factor):
        self.seed = seed
        self.factor = factor
    
    def next(self):
        self.seed = (self.seed * self.factor) % 2147483647
        return self.seed

class Generator2(Generator):
    def __init__(self, seed, factor, div):
        super().__init__(seed, factor)
        self.div = div

    def next(self):
        while True:
            v = super().next()
            if (v % self.div) == 0:
                return v

def countAnswer(genA, genB, iterations):
    answer = 0
    for i in tqdm(range(iterations)):
        a = genA.next()
        b = genB.next()
        if (a & 0xFFFF) == (b & 0xFFFF):
            answer += 1
    return answer

print("Answer #1: %d" % countAnswer(Generator(883, 16807), Generator(879, 48271), 40000000))
print("Answer #2: %d" % countAnswer(Generator2(883, 16807, 4), Generator2(879, 48271, 8), 5000000))