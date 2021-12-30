import itertools
import re
from collections import Counter, namedtuple

re_scanner = re.compile(r'--- scanner (\d+) ---')
re_coord = re.compile(r'(-?\d+),(-?\d+),(-?\d+)')

LineupResult = namedtuple('LineupResult', ['count', 'transform', 'offset'])

transforms = (
    lambda x, y, z: ( x,  y,  z),
    lambda x, y, z: ( x, -z,  y),
    lambda x, y, z: ( x, -y, -z),
    lambda x, y, z: ( x,  z, -y),
    lambda x, y, z: ( y, -x,  z),
    lambda x, y, z: ( y,  z,  x),
    lambda x, y, z: ( y,  x, -z),
    lambda x, y, z: ( y, -z, -x),
    lambda x, y, z: (-x, -y,  z),
    lambda x, y, z: (-x,  z,  y),
    lambda x, y, z: (-x,  y, -z),
    lambda x, y, z: (-x, -z, -y),
    lambda x, y, z: (-y,  x,  z),
    lambda x, y, z: (-y, -z,  x),
    lambda x, y, z: (-y, -x, -z),
    lambda x, y, z: (-y,  z, -x),
    lambda x, y, z: ( z,  y, -x),
    lambda x, y, z: ( z,  x,  y),
    lambda x, y, z: ( z, -y,  x),
    lambda x, y, z: ( z, -x, -y),
    lambda x, y, z: (-z,  y,  x),
    lambda x, y, z: (-z,  x, -y),
    lambda x, y, z: (-z, -y, -x),
    lambda x, y, z: (-z, -x,  y),
)

assert len(transforms) == 24

def read_input(fn):
    scanners, scanner = {}, 0
    with open(fn) as f:
        for l in f:
            midx = re_scanner.match(l)
            if midx:
                scanner = int(midx.group(1))
                scanners[scanner] = []
                continue
            mcoord = re_coord.match(l)
            if mcoord:
                scanners[scanner].append(tuple(map(int, re_coord.findall(l)[0])))
    return scanners

def sub_3d(x, y, z, ox, oy, oz):
    return (x - ox, y - oy, z - oz)

def add_3d(x, y, z, ox, oy, oz):
    return (x + ox, y + oy, z + oz)

def manhattan_3d(a, b):
    return sum(a[i] - b[i] for i in range(3))

def count_identical_points(points1, points2, transform, offset):
    # Relative to points1
    return len([True for a, b in itertools.product(points1, points2) if a == add_3d(*transform(*b), *offset)])

def find_best_lineup(points1, points2):
    # Relative to points1
    global transforms
    
    candidates = []
    
    for t in transforms:
        # List possible offsets
        offsets = [sub_3d(*b1, *t(*b2)) for b1, b2 in itertools.product(points1, points2)]

        if len(offsets) == len(set(offsets)):
            # This transformation is very unlikely as there are no overlapping coordinates at all.
            continue
        
        # Now, the most-occurring difference MUST be the offset. 
        offset = Counter(offsets).most_common(1)[0][0]
            
        c = count_identical_points(points1, points2, t, offset)
        if c > 0:
            candidates.append(LineupResult(c, t, offset))
            
        # For this puzzle we can optimize by just stopping at 12..
        if c >= 12:
            return LineupResult(c, t, offset)

    if len(candidates) == 0:
        return LineupResult(0, None, None)
    return max(candidates, key=lambda x: x.count)

def transform_points(points, offset, transform):
    return [add_3d(*transform(*p), *offset) for p in points]

def part1(scanners):
    Match = namedtuple('Match', ['psci', 'transform', 'offset'])
    points = scanners[0][:]
    done = {0: (0, None, None)}
    scannerabs = {0: (0, 0, 0)}
    checked = []
    
    while len(done) < len(scanners):
        for psci in done.copy().keys():
            for sci, sc in scanners.items():
                if sci in done.keys() or (sci, psci) in checked:
                    continue
                
                res = find_best_lineup(scanners[psci], sc)
                
                checked.append((sci, psci))
                
                if res.count < 12:
                    continue
                    
                # Good match
                done[sci] = Match(psci, res.transform, res.offset)
                    
                # Transform back to relative to scanner 0
                tstep, tpoints, scabs = sci, sc[:], res.offset
                while tstep != 0:
                    # Transform back to parent scanner
                    psc = done[tstep]                    
                    tpoints = transform_points(tpoints, psc.offset, psc.transform)

                    # Transform scanner coordinates
                    if tstep != sci:
                        scabs = add_3d(*psc.transform(*scabs), *psc.offset)

                    tstep = psc.psci

                points += tpoints
                scannerabs[sci] = scabs
                print(f"{len(done)}/{len(scanners)} resolved {sci} from {psci} @ {scabs}")
               
    return set(points), scannerabs
    
def part2(points):
    return max(manhattan_3d(a, b) for a, b in itertools.product(points, points))

inp = read_input('19.txt')
points, scannerabs = part1(inp)
print("Answer #1:", len(points))
print("Answer #2:", part2(scannerabs.values()))
