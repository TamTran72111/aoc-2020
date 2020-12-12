from utilities import read_input
from typing import List


class Group:
    def __init__(self, group: str):
        group = group.splitlines()
        self.num_of_people = len(group)
        self.answers = [0] * 26
        for person in group:
            for answer in person:
                self.answers[ord(answer) - 97] += 1

    def get_total_yes_answers(self):
        return sum([answer > 0 for answer in self.answers])

    def get_everyone_yes(self):
        return sum([answer == self.num_of_people for answer in self.answers])


def part_1(groups: List[Group]):
    return sum([group.get_total_yes_answers() for group in groups])


def part_2(groups: List[Group]):
    return sum([group.get_everyone_yes() for group in groups])


if __name__ == '__main__':
    print('Day 6')
    data = read_input('../inputs/day6.txt')
    groups = [Group(group) for group in data.split('\n\n')]
    print('\tPart 1: {}'.format(part_1(groups)))
    print('\tPart 2: {}'.format(part_2(groups)))
