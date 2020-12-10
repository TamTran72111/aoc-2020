from utilities import read_input

required_fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']


def validate_passport(passportInfo, quickly=True):
    passportInfo = passportInfo.replace('\n', ' ')
    passportInfo = passportInfo.split(' ')
    passport = {}

    for info in passportInfo:
        [key, value] = info.split(':')
        passport[key] = value

    for field in required_fields:
        if field not in passport:
            return False

    if quickly:
        return True

    if int(passport['byr']) < 1920 or int(passport['byr']) > 2002:
        return False

    if int(passport['iyr']) < 2010 or int(passport['iyr']) > 2020:
        return False

    if int(passport['eyr']) < 2020 or int(passport['eyr']) > 2030:
        return False

    try:
        height = int(passport['hgt'][:-2])
    except:
        return False
    if 'cm' in passport['hgt']:
        if height < 150 or height > 193:
            return False
    else:
        if height < 59 or height > 76:
            return False

    if len(passport['hcl']) != 7 or passport['hcl'][0] != '#':
        return False
    for c in passport['hcl'][1:]:
        if c.isdigit() or c in 'abcdef':
            continue
        return False

    if passport['ecl'] not in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']:
        return False

    if len(passport['pid']) != 9 or not passport['pid'].isdigit():
        return False

    return True


def part_1(data):
    return sum(map(validate_passport, data))


def part_2(data):
    return sum([validate_passport(passport, False) for passport in data])


if __name__ == '__main__':
    print('Day 4')
    data = read_input('../inputs/day4.txt')
    passports = data.split('\n\n')
    print('\tPart 1: {}'.format(part_1(passports)))
    print('\tPart 2: {}'.format(part_2(passports)))
