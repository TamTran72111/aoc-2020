import unittest
from utilities import read_blocks
from collections import defaultdict
from typing import List, Dict, Tuple, Optional
from enum import Enum


def rotate(image):
    new_image = []
    n = len(image)
    for i in range(n):
        new_row = ''
        for j in range(n):
            new_row += image[j][n-1-i]
        new_image.append(new_row)
    return new_image


def flip(image):
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

    def share_borders(self, other) -> bool:
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


def construct_share_tiles(tiles: List[str]) -> Tuple[Dict[int, int], Dict[int, Tile]]:
    tiles = [Tile(tile) for tile in tiles]
    tiles = {tile.tile_id: tile for tile in tiles}
    share_tiles = defaultdict(list)

    for tile in tiles.values():
        for other in tiles.values():
            if tile.share_borders(other):
                share_tiles[tile.tile_id].append(other.tile_id)

    return share_tiles, tiles


def validate_assemble(share_tiles: Dict[int, int]) -> int:
    result = 1
    corners = []
    for tile_id in share_tiles:
        if len(share_tiles[tile_id]) == 2:
            result *= tile_id
            corners.append(tile_id)

    return result, corners


def match_top_left(tile, neighbor_0, neighbor_1) -> bool:
    for _ in range(4):
        if tile.match_border(neighbor_0, Border.RIGHT) and tile.match_border(neighbor_1, Border.BOTTOM):
            return True
        if tile.match_border(neighbor_0, Border.BOTTOM) and tile.match_border(neighbor_1, Border.RIGHT):
            return True
        tile.rotate()
    return False


def helper(share_tiles, tiles, tile_id) -> bool:
    tile = tiles[tile_id]
    neighbor_0 = tiles[share_tiles[tile_id][0]]
    neighbor_1 = tiles[share_tiles[tile_id][1]]
    if match_top_left(tile, neighbor_0, neighbor_1):
        return True

    tile.flip()
    return match_top_left(tile, neighbor_0, neighbor_1)


def find_top_left(tiles: Dict[int, Tile], share_tiles: Dict[int, int], corners: List[int]) -> int:

    for tile_id in corners:
        if helper(share_tiles, tiles, tile_id):
            return tile_id

    raise Exception('Should not reach this point')


def remove_border(image: List[List[Tile]]) -> List[str]:
    result = []
    for row in image:
        new_row = [tile.tile_without_borders() for tile in row]
        tmp = []
        for i in range(len(new_row[0])):
            s = ''
            for j in range(len(new_row)):
                s += new_row[j][i]
            tmp.append(s)
        result.extend(tmp)
    return result


def match_next_tile(tiles: Dict[int, Tile], share_tiles, current_id, match_border, adapt_border) -> Optional[int]:
    current_tile = tiles[current_id]
    neighbors = share_tiles[current_id]
    for tile_id in neighbors:
        new_tile = tiles[tile_id]
        if current_tile.match_border(new_tile, match_border):
            new_tile.adapt(
                current_tile.get_border(match_border), adapt_border)
            return tile_id
    return None


def construct_image(tiles: Dict[int, Tile], share_tiles: Dict[int, int], corners: List[int]) -> List[str]:
    top_left_id = find_top_left(tiles, share_tiles, corners)
    image = []
    current_id = top_left_id
    while current_id:
        row = []

        while current_id:
            current_tile = tiles[current_id]
            row.append(current_tile)

            current_id = match_next_tile(
                tiles, share_tiles, current_id, Border.RIGHT, Border.LEFT)
        image.append(row)

        current_id = row[0].tile_id

        current_id = current_id = match_next_tile(
            tiles, share_tiles, current_id, Border.BOTTOM, Border.TOP)

    return remove_border(image)


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


def count_safe(image: List[str]) -> int:
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
        share_tiles, tiles = construct_share_tiles(test_input)
        result, corners = validate_assemble(share_tiles)
        self.assertEqual(result, 20899048083289)

        image = construct_image(tiles, share_tiles, corners)

        self.assertEqual(count_safe(image), 273)

    def test_count_monster(self):
        self.assertEqual(count_monsters(self.monster_image.splitlines()), 2)

    def test_count_safe(self):
        self.assertEqual(count_safe(self.monster_image.splitlines()), 273)
        self.assertEqual(count_safe(self.test_image.splitlines()), 273)


if __name__ == '__main__':
    print('Day 20')
    import time
    start = time.time()
    tiles: List[str] = read_blocks('../inputs/day20.txt')
    share_tiles, tiles = construct_share_tiles(tiles)
    part_1_result, corners = validate_assemble(share_tiles)
    print('\tPart 1: {}'.format(part_1_result))

    print('\tPart 2: {}'.format(count_safe(
        construct_image(tiles, share_tiles, corners))))
    print("\tTime: {:.4f}ms".format((time.time() - start) * 1000))
    # unittest.main()
