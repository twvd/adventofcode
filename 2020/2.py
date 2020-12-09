import re
from collections import Counter

with open('2.txt') as f:
    inp = [l.strip() for l in f.readlines()]

inpr = re.compile('(\d+)\-(\d+) ([a-z])\: ([a-z]+)')

ans1, ans2 = 0, 0

for i in inp:
    min, max, c, s = inpr.findall(i)[0]
    min, max = int(min), int(max)
    cnt = Counter(s)
    if cnt[c] >= min and cnt[c] <= max:
        ans1 += 1
    if (s[min - 1] == c or s[max - 1] == c) and (s[min - 1] != s[max - 1]):
        ans2 += 1

print("Answer #1:", ans1)
print("Answer #2:", ans2)
