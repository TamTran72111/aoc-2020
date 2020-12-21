import unittest
from utilities import read_blocks
from collections import defaultdict
from typing import List, Dict, Tuple, Optional
from enum import Enum

monster_relative_position = [(0, 0), (1, 1), (1, 4), (0, 5), (0, 6),
                             (1, 7), (1, 10), (0, 11), (0, 12), (1, 13),
                             (1, 16), (0, 17), (-1, 18), (0, 18), (0, 19)]


def is_monster_body(image: List[str], row: int, col: int):
    return 0 <= row < len(image) and 0 <= col < len(image[row]) and image[row][col] == '#'


def is_monster_here(image: List[str], row: int, col: int) -> bool:
    return all(map(lambda offset: is_monster_body(image, row+offset[0], col+offset[1]), monster_relative_position))


def count_monsters(image: List[str]) -> int:
    monsters = 0
    for i in range(len(image)):
        for j in range(len(image[0])):
            monsters += 1 if is_monster_here(image, i, j) else 0

    return monsters


def count_safe_position(image: List[str]) -> int:
    total_hash = sum([row.count('#') for row in image])
    for _ in range(4):
        monsters = count_monsters(image)
        if monsters != 0:
            return total_hash - monsters * len(monster_relative_position)
        image = rotate(image)

    image = flip(image)
    for _ in range(4):
        monsters = count_monsters(image)
        if monsters != 0:
            return total_hash - monsters * len(monster_relative_position)
        image = rotate(image)
    return total_hash


def rotate(image: List[str]):
    new_image = []
    n = len(image)
    for i in range(n):
        new_row = ''
        for j in range(n):
            new_row += image[j][n-1-i]
        new_image.append(new_row)
    return new_image


def flip(image: List[str]):
    return [row[::-1] for row in image]


class Border(Enum):
    TOP = 0
    LEFT = 1
    BOTTOM = 2
    RIGHT = 3


class Tile:
    def __init__(self, tile_info: str):
        tile_info = tile_info.splitlines()
        self.tile_id = int(tile_info[0][5:-1])
        self.tile = tile_info[1:]
        self.borders = set()
        # top border
        self.borders.add(tile_info[1])
        self.borders.add(tile_info[1][::-1])
        # right border
        right_border = ''.join([row[-1] for row in tile_info[1:]])
        self.borders.add(right_border)
        self.borders.add(right_border[::-1])
        # bottom border
        self.borders.add(tile_info[-1])
        self.borders.add(tile_info[-1][::-1])
        # left border
        left_border = ''.join([row[0] for row in tile_info[1:]])
        self.borders.add(left_border)
        self.borders.add(left_border[::-1])

    def is_neighbor(self, other) -> bool:
        if other.tile_id == self.tile_id:
            # Same tile
            return False
        return len(self.borders.intersection(other.borders)) > 0

    def get_border(self, border: Border) -> str:
        if border == Border.TOP:
            return self.tile[0]
        if border == Border.BOTTOM:
            return self.tile[-1]
        if border == Border.LEFT:
            index = 0
        else:
            index = -1
        return ''.join([row[index] for row in self.tile])

    def match_border(self, other, border: Border) -> bool:
        border = self.get_border(border)
        return len(other.borders.intersection(set([border]))) > 0

    def rotate(self):
        self.tile = rotate(self.tile)

    def flip(self):
        self.tile = flip(self.tile)

    def __adapt_rotate(self, edge: str, border: Border) -> bool:
        for _ in range(4):
            if edge == self.get_border(border):
                return True
            self.rotate()
        return False

    def adapt(self, edge: str, border: Border):
        if self.__adapt_rotate(edge, border):
            return
        self.flip()
        self.__adapt_rotate(edge, border)

    def tile_without_borders(self) -> List[str]:
        return [row[1:-1] for row in self.tile[1:-1]]

    def __is_top_left(self, right_neighbor, bottom_neighbor) -> bool:
        return self.match_border(right_neighbor, Border.RIGHT) and self.match_border(bottom_neighbor, Border.BOTTOM)

    def is_top_left(self, neighbor_0, neighbor_1) -> bool:
        for _ in range(4):
            if self.__is_top_left(neighbor_0, neighbor_1) or self.__is_top_left(neighbor_1, neighbor_0):
                return True
            self.rotate()
        self.flip()
        for _ in range(4):
            if self.__is_top_left(neighbor_0, neighbor_1) or self.__is_top_left(neighbor_1, neighbor_0):
                return True
            self.rotate()
        return False


class Solution:
    def __init__(self, tiles: List[str]):
        tiles = [Tile(tile) for tile in tiles]
        self.tiles: Dict[int, Tile] = {tile.tile_id: tile for tile in tiles}
        self.neighbors: Dict[int, List[int]] = defaultdict(list)

        for tile in self.tiles.values():
            for other in self.tiles.values():
                if tile.is_neighbor(other):
                    self.neighbors[tile.tile_id].append(other.tile_id)

    def solve_part_1(self) -> int:
        self.corners = []
        result = 1
        for tile_id, neighbors in self.neighbors. items():
            if len(neighbors) == 2:
                result *= tile_id
                self.corners.append(tile_id)

        return result

    def solve_part_2(self) -> int:
        self.__construct_image()
        return count_safe_position(self.image_without_borders)

    def __is_top_left_tile(self, tile_id) -> bool:
        tile = self.tiles[tile_id]
        neighbor_0 = self.tiles[self.neighbors[tile_id][0]]
        neighbor_1 = self.tiles[self.neighbors[tile_id][1]]
        return tile.is_top_left(neighbor_0, neighbor_1)

    def __find_top_left_tile(self) -> int:
        for tile_id in self.corners:
            if self.__is_top_left_tile(tile_id):
                return tile_id

        raise Exception('Should not reach this point')

    def __find_next_tile(self, current_id: int, match_border: Border, adapt_border: Border) -> Optional[int]:
        current_tile = self.tiles[current_id]
        neighbors = self.neighbors[current_id]
        for neighbor_id in neighbors:
            neighbor = self.tiles[neighbor_id]
            if current_tile.match_border(neighbor, match_border):
                neighbor.adapt(
                    current_tile.get_border(match_border), adapt_border)
                return neighbor_id
        return None

    def __construct_row(self, first_tile_in_row: int) -> int:
        current_id = first_tile_in_row
        row = []
        while current_id:
            current_tile = self.tiles[current_id]
            row.append(current_tile)
            current_id = self.__find_next_tile(
                current_id, Border.RIGHT, Border.LEFT)

        self.image_tiles.append(row)
        return self.__find_next_tile(
            row[0].tile_id, Border.BOTTOM, Border.TOP)

    def __construct_image_tiles(self):
        self.image_tiles = []
        first_tile_id = self.__find_top_left_tile()
        while first_tile_id:
            first_tile_id = self.__construct_row(first_tile_id)

    def __construct_image(self):
        self.__construct_image_tiles()
        image_without_borders = []
        for tiles in self.image_tiles:
            tiles_without_borders = [
                tile.tile_without_borders() for tile in tiles]
            tiles_without_borders_concatenated = []
            for i in range(len(tiles_without_borders[0])):
                row = ''
                for j in range(len(tiles_without_borders)):
                    row += tiles_without_borders[j][i]
                tiles_without_borders_concatenated.append(row)
            image_without_borders.extend(tiles_without_borders_concatenated)
        self.image_without_borders = image_without_borders


class Testing(unittest.TestCase):

    test_input = """Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."""

    test_image = """.#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###"""

    monster_image = """.####...#####..#...###..
#####..#..#.#.####..#.#.
.#.#...#.###...#.##.##..
#.#.##.###.#.##.##.#####
..##.###.####..#.####.##
...#.#..##.##...#..#..##
#.##.#..#.#..#..##.#.#..
.###.##.....#...###.#...
#.####.#.#....##.#..#.#.
##...#..#....#..#...####
..#.##...###..#.#####..#
....#.##.#.#####....#...
..##.##.###.....#.##..#.
#...#...###..####....##.
.#.##...#.##.#.#.###...#
#.###.#..####...##..#...
#.###...#.##...#.######.
.###.###.#######..#####.
..##.#..#..#.#######.###
#.#..##.########..#..##.
#.#####..#.#...##..#....
#....##..#.#########..##
#...#.....#..##...###.##
#..###....##.#...##.##.#"""

    def test_part_1(self):
        test_input = self.test_input.split('\n\n')
        solution = Solution(test_input)
        self.assertEqual(solution.solve_part_1(), 20899048083289)

        self.assertEqual(solution.solve_part_2(), 273)

    def test_count_monster(self):
        self.assertEqual(count_monsters(self.monster_image.splitlines()), 2)

    def test_count_safe(self):
        self.assertEqual(count_safe_position(
            self.monster_image.splitlines()), 273)
        self.assertEqual(count_safe_position(
            self.test_image.splitlines()), 273)


if __name__ == '__main__':
    print('Day 20')
    import time
    start = time.time()
    tiles: List[str] = read_blocks('../inputs/day20.txt')

    solution = Solution(tiles)
    print('\tPart 1: {}'.format(solution.solve_part_1()))
    print('\tPart 2: {}'.format(solution.solve_part_2()))

    print("\tTime: {:.4f}ms".format((time.time() - start) * 1000))
    unittest.main()
