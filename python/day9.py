from utilities import read_int_input
from typing import List, Deque
from collections import deque


def two_sum(preamble: Deque[int], target: int) -> bool:
    s = set()
    for number in preamble:
        other = target - number
        if other in s:
            return True
        s.add(number)
    return False


def find_invalid(numbers: List[int]) -> int:
    preamble = deque()
    for number in numbers[:25]:
        preamble.append(number)
    for number in numbers[25:]:
        if not two_sum(preamble, number):
            return number
        preamble.append(number)
        preamble.popleft()
    return -1


def find_min_max_sum(numbers: List[int]) -> int:
    return min(numbers) + max(numbers)


def find_weaknest(numbers: List[int], target: int) -> int:
    contiguous_sum = [numbers[0] + numbers[1]]
    for i in range(2, len(numbers)):
        contiguous_sum = [con_sum + numbers[i] for con_sum in contiguous_sum]
        contiguous_sum.append(numbers[i-1] + numbers[i])
        for index, con_sum in enumerate(contiguous_sum):
            if con_sum == target:
                return find_min_max_sum(numbers[index:i+1])
    return -1


def part_1(numbers: List[int]) -> int:
    return find_invalid(numbers)


def part_2(numbers: List[int]) -> int:
    return find_weaknest(numbers, part_1(numbers))


if __name__ == '__main__':
    print('Day 9')
    numbers = read_int_input('../inputs/day9.txt')
    print('\tPart 1: {}'.format(part_1(numbers)))
    print('\tPart 2: {}'.format(part_2(numbers)))
