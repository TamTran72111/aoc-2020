import unittest
from collections import defaultdict
from typing import List, Set, Tuple

from utilities import read_lines

direction_moves = {
    'e': (1, 0),
    'w': (-1, 0),
    'ne': (0, 1),
    'sw': (0, -1),
    'nw': (-1, 1),
    'se': (1, -1),
}

directions = ['ne', 'nw', 'se', 'sw', 'w', 'e']


def solve_part_1(lines: List[str]) -> Set[Tuple[int, int]]:
    tiles = set()
    for line in lines:
        pos = (0, 0)
        i = 0
        while i < len(line):
            for direction in directions:
                if line[i:i+len(direction)] == direction:
                    move = direction_moves[direction]
                    pos = (pos[0] + move[0], pos[1] + move[1])
                    i += len(direction)
                    break
        if pos in tiles:
            tiles.remove(pos)
        else:
            tiles.add(pos)
    return tiles


def solve_part_2(tiles) -> int:
    for _ in range(100):
        no_flip = set()
        whites = defaultdict(int)
        for pos in tiles:
            count = 0
            for move in direction_moves.values():
                new_pos = (pos[0] + move[0], pos[1] + move[1])
                if new_pos in tiles:
                    # Black neighbour of the current pos
                    count += 1
                else:
                    # White neighbour of the current pos, so increase
                    # the number of black neighbour for this white neighbour
                    # by 1.
                    whites[new_pos] += 1

            if count == 1 or count == 2:
                # This black tile will not be flipped
                no_flip.add(pos)
        white_to_black = set(
            [pos for pos, black_neighbour_count in whites.items() if black_neighbour_count == 2])

        tiles = no_flip.union(white_to_black)
    return len(tiles)


class Testing(unittest.TestCase):

    test_input = """sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"""

    def test_part_1(self):
        tiles = solve_part_1(self.test_input.splitlines())
        self.assertEqual(len(tiles), 10)
        self.assertEqual(solve_part_2(tiles), 2208)


if __name__ == '__main__':
    print('Day 24')
    import time
    start = time.time()
    lines = read_lines('../inputs/day24.txt')

    tiles = solve_part_1(lines)
    print('\tPart 1: {}'.format(len(tiles)))
    print('\tPart 2: {}'.format(solve_part_2(tiles)))

    print("\tTime: {:.4f}ms".format((time.time() - start) * 1000))
    unittest.main()
