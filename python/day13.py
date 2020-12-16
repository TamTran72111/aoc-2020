from utilities import read_lines
from typing import List, Tuple


def part_1(earliest_depart: int, buses: List[Tuple[int, int]]) -> int:
    wait = 10_000_00
    bus_id = None
    for _, bus in buses:
        if earliest_depart % bus == 0:
            bus_id = bus
            wait = 0
        else:
            depart = (earliest_depart // bus + 1) * bus
            if depart - earliest_depart < wait:
                wait = depart - earliest_depart
                bus_id = bus
    return wait * bus_id


def part_2(buses: List[Tuple[int, int]]) -> int:
    result = 0
    product = 1
    for _, bus in buses:
        product *= bus

    for index, bus in buses:
        b = (bus - index % bus) % bus
        n = product // bus
        x = 1
        xi = n
        while xi % bus != 1:
            xi += n
            x += 1

        result += b * n * x

    return result % product


if __name__ == '__main__':
    print('Day 13')
    data = read_lines('../inputs/day13.txt')
    earliest_depart = int(data[0])
    buses = data[1].split(',')
    buses = filter(lambda x: x[1] != 'x', enumerate(buses))
    buses = list(map(lambda x: (x[0], int(x[1])), buses))
    print('\tPart 1: {}'.format(part_1(earliest_depart, buses)))
    print('\tPart 2: {}'.format(part_2(buses)))
