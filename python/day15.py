import unittest
from typing import List


def memory_game(starting_numbers: List[int], last_turn: int) -> int:
    memory = [None] * 30_000_001
    last_num = starting_numbers[-1]
    for index, num in enumerate(starting_numbers[:-1]):
        memory[num] = index
    for turn in range(len(starting_numbers)-1, last_turn - 1):
        if memory[last_num] is None:
            memory[last_num] = turn
            last_num = 0
        else:
            new_last_num = turn - memory[last_num]
            memory[last_num] = turn
            last_num = new_last_num

    return last_num


def part_1() -> int:
    return memory_game([1, 20, 8, 12, 0, 14], 2020)


def part_2() -> int:
    return memory_game([1, 20, 8, 12, 0, 14], 30000000)


class Testing(unittest.TestCase):

    def test_small_turn(self):
        starting_numbers = [0, 3, 6]
        self.assertEqual(memory_game(starting_numbers, 4), 0)
        self.assertEqual(memory_game(starting_numbers, 5), 3)
        self.assertEqual(memory_game(starting_numbers, 6), 3)
        self.assertEqual(memory_game(starting_numbers, 7), 1)
        self.assertEqual(memory_game(starting_numbers, 8), 0)
        self.assertEqual(memory_game(starting_numbers, 9), 4)
        self.assertEqual(memory_game(starting_numbers, 10), 0)

    def test_2020(self):
        self.assertEqual(memory_game([0, 3, 6], 2020), 436)
        self.assertEqual(memory_game([1, 3, 2], 2020), 1)
        self.assertEqual(memory_game([2, 1, 3], 2020), 10)
        self.assertEqual(memory_game([1, 2, 3], 2020), 27)
        self.assertEqual(memory_game([2, 3, 1], 2020), 78)
        self.assertEqual(memory_game([3, 2, 1], 2020), 438)
        self.assertEqual(memory_game([3, 1, 2], 2020), 1836)

    def test_large_turn(self):
        self.assertEqual(memory_game([0, 3, 6], 30000000), 175594)
        self.assertEqual(memory_game([1, 3, 2], 30000000), 2578)
        self.assertEqual(memory_game([2, 1, 3], 30000000), 3544142)
        self.assertEqual(memory_game([1, 2, 3], 30000000), 261214)
        self.assertEqual(memory_game([2, 3, 1], 30000000), 6895259)
        self.assertEqual(memory_game([3, 2, 1], 30000000), 18)
        self.assertEqual(memory_game([3, 1, 2], 30000000), 362)


if __name__ == '__main__':
    print('Day 15')
    print('\tPart 1: {}'.format(part_1()))
    print('\tPart 2: {}'.format(part_2()))
    # unittest.main()
