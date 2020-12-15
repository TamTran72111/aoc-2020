from utilities import read_int_input
from collections import defaultdict
from typing import List, Dict


def count_jolt_differences(adapters: List[int]) -> Dict[int, int]:
    differences = defaultdict(int)
    # difference of the device and the highest jolt adpater
    differences[3] = 1
    prev = 0
    for adapter in adapters:
        differences[adapter - prev] += 1
        prev = adapter
    return differences


def find_number_of_arrangements(adapters: List[int]) -> int:
    jolts = [0] + adapters
    arrangements = [0] * len(jolts)
    arrangements[0] = 1
    for i in range(1, len(arrangements)):
        for j in range(1, 4):
            if i - j > -1 and jolts[i] - 3 <= jolts[i - j]:
                arrangements[i] += arrangements[i - j]
    return arrangements[-1]


def part_1(adapters: List[int]) -> int:
    differences = count_jolt_differences(adapters)
    return differences[1] * differences[3]


def part_2(adapters: List[int]) -> int:
    return find_number_of_arrangements(adapters)


if __name__ == '__main__':
    print('Day 10')
    adapters = read_int_input('../inputs/day10.txt')
    adapters.sort()
    print('\tPart 1: {}'.format(part_1(adapters)))
    print('\tPart 2: {}'.format(part_2(adapters)))
