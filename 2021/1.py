with open('1.txt') as f:
    inp = [int(l.strip()) for l in f.readlines()]

def increments(a: [int]) -> int:
    answer = 0
    last = a[0]
    for i in a[1:]:
        if i > last:
            answer += 1
        last = i
    return answer
    
def rolling_window(a: [], window_size: int):
    for i in range(len(a) - window_size + 1):
        yield a[i:i+window_size]

print("Answer #1:", increments(inp))
print("Answer #2:", increments([sum(x) for x in rolling_window(inp, 3)]))
