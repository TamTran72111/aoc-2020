from utilities import read_lines
from functools import reduce


def count_trees(data, moves):
    trees = 0
    row = 0
    col = 0
    while row < len(data):
        if data[row][col] == '#':
            trees += 1
        row += moves[1]
        col += moves[0]
        col %= len(data[0])
    return trees


def part_1(data):
    return count_trees(data, [3, 1])


def part_2(data):
    moves = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
    return reduce(lambda x, y: x * y, [count_trees(data, move) for move in moves])


if __name__ == '__main__':
    print('Day 3')
    data = read_lines('../inputs/day3.txt')
    print('\tPart 1: {}'.format(part_1(data)))
    print('\tPart 2: {}'.format(part_2(data)))
