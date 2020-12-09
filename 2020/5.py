with open('5.txt') as f:
    seatids = [int(l.replace('F','0').replace('B', '1').replace('L', '0').replace('R', '1'), 2) for l in f.readlines()]

print("Answer #1: ", max(seatids))
p = list(range(min(seatids), max(seatids)))
print("Answer #2: ", (set(p) - set(seatids)))
