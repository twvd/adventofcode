import re

passports = []

with open('4.txt') as f:
    # Read and normalize input to space seperated fields
    inp = [l.strip().replace('\n', ' ') for l in f.read().split('\n\n')]
    for p in [i.split(' ') for i in inp]:
        passport = {}
        for i in p:
            pr = i.split(':')
            passport[pr[0]] = pr[1]
        passports.append(passport)
        
    
print("Total: ", len(passports))

def check_ans1(p):
    if len(p.keys()) == 8:
        return True
    if len(p.keys()) == 7 and 'cid' not in p.keys():
        return True
    return False

ans1 = [p for p in passports if check_ans1(p)]
print("Answer #1: ", len(ans1))

def check_ans2(p):
    """
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
      If cm, the number must be at least 150 and at most 193.
      If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
    """
    if int(p['byr']) < 1920 or int(p['byr']) > 2002:
        return False
    
    if int(p['iyr']) < 2010 or int(p['iyr']) > 2020:
        return False
    
    if int(p['eyr']) < 2020 or int(p['eyr']) > 2030:
        return False
    
    if p['hgt'][-2:] not in ('in', 'cm'):
        return False
    
    hgt = int(p['hgt'][:-2])
    hgtunit = p['hgt'][-2:]
    if hgtunit == 'cm' and (hgt > 193 or hgt < 150):
        return False
    
    if hgtunit == 'in' and (hgt > 76 or hgt < 59):
        return False
    
    if not re.search(r'^#([0-9a-f]{6})$', p['hcl']):
        return False
    
    if p['ecl'] not in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'):
        return False
    
    if not re.search(r'^[0-9]{9}$', p['pid']):
        return False
    
    return True

print("Answer #2: ", len([p for p in ans1 if check_ans2(p)]))
