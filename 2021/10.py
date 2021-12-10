with open('10.txt') as f:
    inp = [l.strip() for l in f.readlines()]

pairs = {
    '{': '}',
    '[': ']',
    '<': '>',
    '(': ')'
}

points_part1 = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137
}

points_part2 = {
    '(': 1,
    '[': 2,
    '{': 3,
    '<': 4
}

answer1 = 0
scores = []
for l in inp:
    stack = []
    for c in l:
        if len(stack) > 0 and c in pairs.values() and pairs[stack[-1]] == c:
            del stack[-1]
        elif c in pairs.keys():
            stack += [c]
        else:
            answer1 += points_part1[c]
            break
    else:
        score = 0
        for c in reversed(stack):
            score = score * 5 + points_part2[c]
        if score > 0:
            scores += [score]

print("Answer #1:", answer1)
print("Answer #2:", sorted(scores)[len(scores) // 2])
