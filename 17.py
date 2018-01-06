inp = 382
buffer = [0]
p = 0

for i in range(1, 2017+1):
    p = ((p + inp) % i) + 1
    buffer.insert(p, i)

print("Answer #1: %d" % buffer[buffer.index(2017) + 1])

answer2 = 0
p = 0
for i in range(1, 50000000+1):
    p = ((p + inp) % i) + 1
    if p == 1:
        answer2 = i

print("Answer #2: %d" % answer2)