import knothash

with open('10.txt') as f:
    inp = f.readline().strip()

nums = inp.split(',')
nums = list(map(int, nums))

hash = knothash.knotHash1(nums)
print("Answer #1: %d" % (hash[0] * hash[1]))
print("Answer #2: %s" % knothash.knotHash2Str(inp))