import unittest
from utilities import read_lines
from collections import defaultdict
from typing import List


def get_new_cube_state(state, x, y, z, w):
    count_active = 0
    r = [-1, 0, 1]
    for i in r:
        for j in r:
            for k in r:
                for l in r:
                    if i == 0 and j == 0 and k == 0 and l == 0:
                        continue
                    if (x+i, y+j, z+k, w+l) in state:
                        count_active += 1
    return count_active == 3 or ((x, y, z, w) in state and count_active == 2)


def part_1(initial: List[str], four_dims=False) -> int:
    state = set()
    new_state = set()
    for y, row in enumerate(initial):
        for x, value in enumerate(row):
            if value == '#':
                state.add((x, y, 0, 0))

    min_x = min_y = min_z = min_w = -1
    max_x, max_y, max_z, max_w = len(initial[0]), len(initial), 1, 1

    if not four_dims:
        min_w = max_w = 0

    for _ in range(6):
        for x in range(min_x, max_x + 1):
            for y in range(min_y, max_y + 1):
                for z in range(min_z, max_z + 1):
                    for w in range(min_w, max_w + 1):
                        if get_new_cube_state(state, x, y, z, w):
                            new_state.add((x, y, z, w))
                        elif (x, y, z, w) in new_state:
                            new_state.remove((x, y, z, w))

        min_x -= 1
        min_y -= 1
        min_z -= 1
        max_x += 1
        max_y += 1
        max_z += 1
        if four_dims:
            min_w -= 1
            max_w += 1
        new_state, state = state, new_state

    return len(state)


def part_2(initial: List[str]) -> int:
    return part_1(initial, True)


class Testing(unittest.TestCase):

    def test_part_1(self):
        initial = [".#.",
                   "..#",
                   "###"]
        self.assertEqual(part_1(initial), 112)

    def test_part_2(self):
        initial = [".#.",
                   "..#",
                   "###"]
        self.assertEqual(part_2(initial), 848)


if __name__ == '__main__':
    print('Day 17')
    initial: List[str] = read_lines('../inputs/day17.txt')

    print('\tPart 1: {}'.format(part_1(initial)))
    print('\tPart 2: {}'.format(part_2(initial)))
    # unittest.main()
