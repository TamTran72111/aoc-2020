from utilities import read_lines


binary_values = {
    'F': '0',
    'B': '1',
    'L': '0',
    'R': '1'
}


def convert_id(boarding_pass):
    for key, value in binary_values.items():
        boarding_pass = boarding_pass.replace(key, value)
    row = int(boarding_pass[:7], 2)
    col = int(boarding_pass[7:], 2)
    return row * 8 + col


def part_1(ids):
    return max(ids)


def part_2(ids):
    ids = sorted(ids)

    for i in range(len(ids) - 1):
        if ids[i] + 1 != ids[i+1]:
            return ids[i] + 1

    raise Exception("This is unreachable")


if __name__ == '__main__':
    print('Day 5')
    data = read_lines('../inputs/day5.txt')
    ids = list(map(convert_id, data))
    print('\tPart 1: {}'.format(part_1(ids)))
    print('\tPart 2: {}'.format(part_2(ids)))
