import unittest
import time


def solve_part_1(public_card: int, public_door: int) -> int:
    loop_size = 0
    value = 1
    while value != public_card:
        value *= 7
        value %= 20201227
        loop_size += 1

    return pow(public_door, loop_size, 20201227)


class Testing(unittest.TestCase):

    test_input = """"""

    def test_part_1(self):
        public_card = 5764801
        public_door = 17807724
        self.assertEqual(solve_part_1(public_card, public_door), 14897079)


if __name__ == '__main__':
    print('Day 25')
    start = time.time()
    public_card = 8335663
    public_door = 8614349

    print('\tPart 1: {}'.format(solve_part_1(public_card, public_door)))

    print("\tTime: {:.4f}ms".format((time.time() - start) * 1000))
    unittest.main()
