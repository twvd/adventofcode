import statistics

with open('7.txt') as f:
    crabs = list(map(int, f.readline().split(',')))

median = int(statistics.median(crabs))
print("Answer #1:", sum(abs(i - median) for i in crabs))

mean = int(statistics.mean(crabs))
print("Answer #2:", sum(abs(i - mean) * (abs(i - mean) + 1) // 2 for i in crabs))
