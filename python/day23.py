import array
import unittest
from typing import List


def perform_move(circle: List[int], current_cup: int, wrap_value: int) -> int:
    pick_1 = circle[current_cup]
    pick_2 = circle[pick_1]
    pick_3 = circle[pick_2]
    # now pickup is the value after the pick up 3 immediately cups after the
    # current cup, which mean it is the next current cup
    circle[current_cup] = circle[pick_3]

    # find destination value
    destination = current_cup - 1 if current_cup != 1 else wrap_value
    while destination == pick_1 or destination == pick_3 or destination == pick_2:
        destination -= 1
    if destination == 0:
        destination = wrap_value

    # insert the 3 cups picked up after the destination, so the next cup of
    # the destination cup will be the first picked up cup, and the next cup
    # of the last picked up cup will be the cup next to the destination cup
    # before inserting
    circle[pick_3] = circle[destination]
    circle[destination] = pick_1

    # return the cup after the current cup, which is the current cup for the
    # next move
    return circle[current_cup]


def solve_part_1(initial: str, moves: int) -> str:
    # Setup the circle
    circle = [0] * 10
    nums = [int(n) for n in initial]
    nums.append(nums[0])
    for i in range(9):
        circle[nums[i]] = nums[i+1]
    current_cup = nums[0]

    for _ in range(moves):
        current_cup = perform_move(circle, current_cup, 9)

    # Construct the result string
    result = ''
    current = circle[1]
    for _ in range(8):
        result += str(current)
        current = circle[current]
    return result


def solve_part_2(initial: str) -> int:
    circle = array.array('I', range(1, 1_000_002))
    for i in range(10, 1_000_000):
        circle[i] = i + 1

    nums = [int(n) for n in initial]
    for i in range(8):
        circle[nums[i]] = nums[i+1]
    circle[nums[-1]] = 10
    circle[-1] = nums[0]  # Last cup point to the first cup, to form a circle
    current_cup = nums[0]

    for _ in range(10_000_000):
        current_cup = perform_move(circle, current_cup, 1_000_000)

    return circle[1] * circle[circle[1]]


class Testing(unittest.TestCase):

    def test_part_1(self):
        test_input = "389125467"

        self.assertEqual(solve_part_1(test_input, 10), "92658374")
        self.assertEqual(solve_part_1(test_input, 100), "67384529")

    def test_part_2(self):
        test_input = "389125467"

        self.assertEqual(solve_part_2(test_input), 149245887792)


if __name__ == '__main__':
    print('Day 23')
    import time
    start = time.time()
    initial = "538914762"

    print('\tPart 1: {}'.format(solve_part_1(initial, 100)))
    print('\tPart 2: {}'.format(solve_part_2(initial)))

    print("\tTime: {:.4f}ms".format((time.time() - start) * 1000))
    unittest.main()
