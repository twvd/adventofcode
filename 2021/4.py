import itertools
import re

WL = 5

class Bingocard:
    def __init__(self, raw):
        self.card = [[int(cell.strip()) for cell in re.findall(r'(\d+)', row)] for row in raw]
        self.state = [[False for i in range(WL)] for j in range(WL)]
    
    def draw(self, i):
        for x, y in itertools.product(range(WL), range(WL)):
            if self.card[y][x] == i:
                self.state[y][x] = True
                self.last = i
                break
    
    def won(self):
        return any(all(row) for row in self.state) or any(all(col) for col in zip(*self.state))
        
    def unmarked(self):
        return [self.card[y][x] for x, y in itertools.product(range(WL), range(WL)) if not self.state[y][x]]
             
with open('4.txt') as f:
    inp = [int(i) for i in f.readline().split(',')]
    f.readline()
    
    rawcards = [l.strip() for l in f.readlines()]
    cards = [Bingocard(rawcards[i:i+WL]) for i in range(0, len(rawcards), WL+1)]

done = []
for i in inp:
    for c in set(cards) - set(done):
        c.draw(i)
    done = done + [c for c in cards if c.won() and c not in done]

print("Answer #1:", sum(done[0].unmarked()) * done[0].last)
print("Answer #2:", sum(done[-1].unmarked()) * done[-1].last)
