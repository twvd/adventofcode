with open('6.txt') as f:
    inp = []
    for l in f.read().split('\n\n'):
        inp.append([set(x) for x in l.split('\n')])
    
print("Answer #1: ", sum([len(set().union(*x)) for x in inp]))
print("Answer #2: ", sum([len(x[0].intersection(*x)) for x in inp]))
