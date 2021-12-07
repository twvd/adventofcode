import statistics

with open('7.txt') as f:
    crabs = list(map(int, f.readline().split(',')))

median = int(statistics.median(crabs))
print("Answer #1:", sum(abs(i - median) for i in crabs))

last = 2**64
for j in set(sorted(crabs)):
    n = sum(abs(i - j) * (abs(i - j) + 1) // 2 for i in crabs)
    if n > last:
        break
    last = n

print("Answer #2:", last)
