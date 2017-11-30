# Advent Warmup: Elvish Cheat Codes
# ---------------------------------
# In the run up to advent, the elves are playing on their video game console. Before long, one of the elves manages to discover a cheat mode, by entering a sequence of button presses on the controller.
# The sequence involves the buttons 'Up', 'Down', 'Left', 'Right', 'A', 'B', and terminates with a single press of the 'Start' button.
# The elves begin to ponder the significance of the sequence they discovered, and decide to draw a chart.
# Starting at the origin in an (x,y) grid, the buttons Up, Down, Left, Right are imagined to move a cursor a single step in the corresponding direction through the grid. Buttons A and B place corresponding markers at the current cursor location.
# -------------------------------------------------------------
# Example:  Up, A, Right, Right, B, Left, B, Start
# Taking Right to be the positive x direction, and Up to be the positive y direction. This sequence will move one step up from the origin (0,0), and place an 'A' marker at location (0,1), then a 'B' marker at location (2,1). The cursor will move one step left and another 'B' marker is placed at location (1,1). Then the cursor halts at location (1,1).
# -------------------------------------------------------------
# Example:  Up, Up, Down, Down, Left, Right, Left, Right, B, A, Start
# Again, starting from the origin (0,0), this sequence will place both a 'B' and an 'A' marker at (0,0), and the cursor will halt at (0,0).
# -------------------------------------------------------------
# The taxicab distance ( https://en.wikipedia.org/wiki/Taxicab_distance ) between two grid locations is defined as the (positive) difference between the two points' x values + the (positive) difference between their y values.
# eg, between locations (1,2) and (8,6), the difference between the two x values (1 and 8) is 7, and the difference between the two y values (2 and 6) is 4. Therefore, the taxicab distance is 7 + 4 = 11.
# Your input is here: https://pastebin.com/wGmzZHeq

# Question 1: 
# Identify the marker furthest from the origin, as measured by the taxicab distance, and return that distance.

# Question 2: 
# Consider all pairs of *different* markers (where a pair may consist of any 'A' and any 'B' marker). Identify the pair maximally far apart from one another, as measured by the taxicab distance, and return that distance.

import itertools

def distance(p1, p2):
    x = abs(p1[0] - p2[0])
    y = abs(p1[1] - p2[1])
    return x + y

origin = [0, 0]

p = origin.copy()
a = p.copy()
b = p.copy()

aarray = []
barray = []

with open('warmup.txt') as f:
    inp = f.readline().split(', ')

answer1 = 0
for s in inp:
    if s == "Down": p[0] -= 1
    elif s == "Up": p[0] += 1
    elif s == "Left": p[1] -= 1
    elif s == "Right": p[1] += 1
    elif s == "B":
        b = p.copy()
        barray.append(b)
        answer1 = max(answer1, distance(b, origin))
    elif s == "A":
        a = p.copy()
        aarray.append(a)
        answer1 = max(answer1, distance(a, origin))

print("Answer 1: %d" % answer1)

answer2 = 0
for a, b in itertools.product(aarray, barray):
    answer2 = max(answer2, distance(a, b))

print("Answer 2: %d" % answer2)