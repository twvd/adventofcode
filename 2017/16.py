import string
import itertools

def dance(moves, programs):
    programs = programs[:]
    for move in moves:
        if move[0] == 's':
            size = int(move[1:])
            programs = programs[-size:] + programs[:-size]
        elif move[0] == 'x':
            m = move[1:].split('/')
            i1, i2 = int(m[0]), int(m[1])
            programs[i1], programs[i2] = programs[i2], programs[i1]
        elif move[0] == 'p':
            i1 = programs.index(move[1])
            i2 = programs.index(move[3])
            programs[i1], programs[i2] = programs[i2], programs[i1]
    return programs

def answer2(moves, programs, total):
    encountered = []
    for i in itertools.count():
        if programs in encountered:
            print("%d cycles in each sequence (answer idx = %d)" % (i, total % i))
            return encountered[total % i]
        encountered.append(programs)
        programs = dance(moves, programs)

programs = list(string.ascii_lowercase[0:16])

with open('16.txt') as f:
    moves = f.readline().strip().split(',')

print("Answer #1: %s" % ''.join(dance(moves, programs)))
print("Answer #2: %s" % ''.join(answer2(moves, programs, 1000000000)))