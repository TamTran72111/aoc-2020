from utilities import read_lines
from typing import List


class Layout:
    def __init__(self, layout: List[str]):
        self.layout_original = [[ch for ch in row] for row in layout]
        self.directions = [(-1, -1), (-1, 0), (-1, 1),
                           (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]

    def check_valid_position(self, row: int, col: int) -> bool:
        return row >= 0 and col >= 0 and row < len(self.layout) and col < len(self.layout[0])

    def check_occupied(self, row: int, col: int, direction: (int, int), first_visible: bool) -> bool:
        row += direction[0]
        col += direction[1]
        if self.check_valid_position(row, col):
            if self.layout[row][col] == '#':
                return True
            if self.layout[row][col] == '.' and first_visible:
                return self.check_occupied(row, col, direction, first_visible)
        return False

    def count_occupied_around(self, row: int, col: int, first_visible: bool) -> int:
        ans = 0
        for direction in self.directions:
            if self.check_occupied(row, col, direction, first_visible):
                ans += 1

        return ans

    def count_occupied_seat(self, first_visible: bool = False, rule: int = 4) -> int:
        self.layout = [[ch for ch in row] for row in self.layout_original]
        new_layout = [[i for i in row] for row in self.layout]
        changed = True

        while changed:
            changed = False

            for i in range(len(self.layout)):
                for j in range(len(self.layout[0])):
                    if self.layout[i][j] == '.':
                        continue
                    occupied_around = self.count_occupied_around(
                        i, j, first_visible)
                    if self.layout[i][j] == 'L' and occupied_around == 0:
                        new_layout[i][j] = '#'
                        changed = True
                    elif self.layout[i][j] == '#' and occupied_around >= rule:
                        new_layout[i][j] = 'L'
                        changed = True
                    else:
                        new_layout[i][j] = self.layout[i][j]
            new_layout, self.layout = self.layout, new_layout

        return sum([sum([i == '#' for i in row]) for row in self.layout])


def part_1(layout: Layout) -> int:
    return layout.count_occupied_seat()


def part_2(layout: Layout) -> int:
    return layout.count_occupied_seat(True, 5)


if __name__ == '__main__':
    print('Day 11')
    layout = Layout(read_lines('../inputs/day11.txt'))
    print('\tPart 1: {}'.format(part_1(layout)))
    print('\tPart 2: {}'.format(part_2(layout)))
