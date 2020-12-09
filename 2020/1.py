import itertools

with open('1.txt') as f:
    inp = [int(l) for l in f.readlines()]
  
for i, j in itertools.product(inp, inp):
    if i + j == 2020:
        print(i, j, i*j)
        break

for i, j, k in itertools.product(inp, inp, inp):
    if i+j+k == 2020:
        print(i,j,k,i*j*k)
        break
