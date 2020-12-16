from utilities import read_lines
from typing import List

directions = {'N': 0, 'E': 1, 'S': 2, 'W': 3}
direction_factors = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def calculate_manhattan_distance(instructions: List[str]) -> int:
    # Starting position
    x, y = 0, 0

    # Starting direction is east
    direction_index = directions['E']

    for instruction in instructions:
        value = int(instruction[1:])
        action = instruction[0]
        if action in directions:
            direction = direction_factors[directions[action]]
            x += value * direction[0]
            y += value * direction[1]
        elif action == 'F':
            x += value * direction_factors[direction_index][0]
            y += value * direction_factors[direction_index][1]
        else:
            factor = -1
            if action == 'R':
                factor = 1
            direction_index += (value // 90) * factor + 4
            direction_index %= 4

    return abs(x) + abs(y)


def calculate_manhattan_distance_with_waypoint(instructions):
    # Starting position
    x, y = 0, 0
    way_x, way_y = 10, 1

    for instruction in instructions:
        value = int(instruction[1:])
        action = instruction[0]

        if action in directions:
            direction = direction_factors[directions[action]]
            way_x += value * direction[0]
            way_y += value * direction[1]
        elif action == 'F':
            x += way_x * value
            y += way_y * value
        elif value == 180:
            way_x *= -1
            way_y *= -1
        else:
            rotation = value
            if action == 'L':
                rotation = 360 - value

            factor_x = 1 if rotation == 90 else -1
            factor_y = -1 if rotation == 90 else 1

            way_x, way_y = factor_x * way_y, factor_y * way_x

    return abs(x) + abs(y)


def part_1(instructions: List[str]) -> int:
    return calculate_manhattan_distance(instructions)


def part_2(instructions: List[str]) -> int:
    return calculate_manhattan_distance_with_waypoint(instructions)


if __name__ == '__main__':
    print('Day 12')
    instructions = read_lines('../inputs/day12.txt')
    print('\tPart 1: {}'.format(part_1(instructions)))
    print('\tPart 2: {}'.format(part_2(instructions)))
