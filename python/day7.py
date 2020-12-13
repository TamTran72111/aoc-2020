from utilities import read_lines
from collections import defaultdict, deque
from typing import List, Dict


class RegulationGraph:
    def __init__(self, regulations: List[str]):
        self.parent_bags = defaultdict(list)
        self.inner_bags = defaultdict(list)
        for regulation in regulations:
            self.read_regulation(regulation)
        self.mem = {}

    def read_regulation(self, regulation: str):
        # Ignore the `.` at the end of line
        regulation = regulation[:-1]

        # Split into the outer bags and inner bags
        parts = regulation.split(' contain ')

        # Ignore the word ` bags`
        outer_bag = parts[0][:-5]

        inner_bags = parts[1].split(', ')
        for bag in inner_bags:
            if 'no other' in bag:
                continue
            color_start = bag.find(' ') + 1
            color_end = bag.rfind(' ')
            color = bag[color_start:color_end]

            self.parent_bags[color].append(outer_bag)

            number_of_inner_bag = int(bag[:color_start])
            self.inner_bags[outer_bag].append((number_of_inner_bag, color))

    def count_outermost_bag(self, color):
        s = set()
        q = deque()
        q.append(color)
        while len(q) != 0:
            color = q.popleft()
            for parent in self.parent_bags[color]:
                if parent not in s:
                    s.add(parent)
                    q.append(parent)
        return len(s)

    def count_bag_inside(self, color):
        mem = self.mem

        def helper(color):
            if mem.get(color) is None:
                if len(self.inner_bags[color]) == 0:
                    mem[color] = 0
                else:
                    ans = 0
                    for inner_bag in self.inner_bags[color]:
                        ans += inner_bag[0]
                        ans += inner_bag[0] * helper(inner_bag[1])
                    mem[color] = ans
            return mem.get(color)
        return helper(color)


def part_1(graph: RegulationGraph):
    return graph.count_outermost_bag('shiny gold')


def part_2(graph: RegulationGraph):
    return graph.count_bag_inside('shiny gold')


if __name__ == '__main__':
    print('Day 7')
    data = read_lines('../inputs/day7.txt')
    graph = RegulationGraph(data)
    print('\tPart 1: {}'.format(part_1(graph)))
    print('\tPart 2: {}'.format(part_2(graph)))
