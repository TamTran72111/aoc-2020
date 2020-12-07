
def read_input():
    with open('../inputs/day1.txt') as f:
        data = f.read()
    return map(int, data.splitlines())


def two_sum(data, target):
    s = set()
    for number in data:
        other = target - number
        if other in s:
            return other * number
        s.add(number)
    return None


def part_1(data):
    return two_sum(data, 2020)


def part_2(data):
    for index, number in enumerate(data):
        result = two_sum(data[index+1:], 2020 - number)
        if result:
            return number * result


def fix_expense_report():
    print('Day 1')
    data = read_input()
    print('\tPart 1: {}'.format(part_1(data)))
    print('\tPart 2: {}'.format(part_2(data)))


if __name__ == '__main__':
    fix_expense_report()
