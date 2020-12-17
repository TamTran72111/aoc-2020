import unittest
from utilities import read_input
from collections import defaultdict
from typing import List, Dict, Tuple

test_input = """class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"""

test_input2 = """class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"""


class FieldRule:
    def __init__(self, rule: str):
        rule: List[str] = rule.split(' or ')
        self.first_range: Tuple[int, int] = self._read_range(rule[0])
        self.second_range: Tuple[int, int] = self._read_range(rule[1])

    def _read_range(self, range: str) -> Tuple[int, int]:
        range: List[str, str] = range.split('-')
        return int(range[0]), int(range[1])

    def validate(self, value: int) -> bool:
        return self.first_range[0] <= value <= self.first_range[1] or\
            self.second_range[0] <= value <= self.second_range[1]


class TicketRule:
    def __init__(self, rules: str):
        self.rules: Dict[str, FieldRule] = {}
        self.rule_index: Dict[str, int] = {}
        self.valid_tickets: List[List[int]] = []
        for rule in rules.splitlines():
            self.set_rule(rule)

    def set_rule(self, rule: str):
        parts: List[str] = rule.split(': ')
        field: str = parts[0]
        self.rules[field] = FieldRule(parts[1])

    def _validate_value(self, value: int) -> bool:
        return any(map(lambda rule: rule.validate(value), self.rules.values()))

    def count_invalid(self, ticket: List[int]) -> int:
        invalid = 0
        valid = True
        for value in ticket:
            if not self._validate_value(value):
                valid = False
                invalid += value
        if valid:
            self.valid_tickets.append(ticket)
        return invalid

    def validate_rule(self, field: str, ticket_idx: int) -> bool:
        rule: FieldRule = self.rules[field]
        for ticket in self.valid_tickets:
            if not rule.validate(ticket[ticket_idx]):
                return False
        return True

    def detect_index(self):
        # Find the possible indices for each field
        possile_idx: Dict[str, List[int]] = defaultdict(list)
        for field in self.rules:
            for i in range(len(self.rules)):
                if self.validate_rule(field, i):
                    possile_idx[field].append(i)

        # Sort by len of possible choices to help find faster.
        possible_idx_len = [(len(possile_idx[field]), field)
                            for field in self.rules]
        possible_idx_len.sort()

        used_indices = [False] * len(self.rules)

        def helper(field_idx):
            if field_idx == len(self.rules):
                return True

            field = possible_idx_len[field_idx][1]
            for idx in possile_idx[field]:
                if not used_indices[idx]:
                    used_indices[idx] = True
                    if helper(field_idx + 1):
                        self.rule_index[field] = idx
                        return True
                    used_indices[idx] = False

        helper(0)


def interpret_data(data: str) -> Tuple[TicketRule, List[int], List[List[int]]]:
    parts: List[str] = data.split('\n\n')
    rule: TicketRule = TicketRule(parts[0])

    my_ticket: str = parts[1].splitlines()[1]
    my_ticket = [int(value) for value in my_ticket.split(',')]

    nearby_tickets: List[str] = parts[2].splitlines()[1:]
    nearby_tickets: List[List[int]] = [
        [int(value) for value in ticket.split(',')] for ticket in nearby_tickets]

    return rule, my_ticket, nearby_tickets


def part_1(rule: TicketRule, nearby_tickets: List[List[int]]) -> int:
    return sum([rule.count_invalid(ticket) for ticket in nearby_tickets])


def part_2(rule: TicketRule, my_ticket: List[int]) -> int:
    rule.detect_index()
    indices = []
    for field in rule.rules:
        if 'departure' in field:
            indices.append(rule.rule_index[field])

    result = 1
    for idx in indices:
        result *= my_ticket[idx]
    return result


class Testing(unittest.TestCase):

    def test_field_range(self):
        rule = FieldRule("1-3 or 5-7")

        self.assertTrue(rule.validate(1))
        self.assertTrue(rule.validate(2))
        self.assertTrue(rule.validate(3))
        self.assertTrue(rule.validate(5))
        self.assertTrue(rule.validate(6))
        self.assertTrue(rule.validate(7))

        self.assertFalse(rule.validate(0))
        self.assertFalse(rule.validate(4))
        self.assertFalse(rule.validate(8))
        self.assertFalse(rule.validate(100))

    def test_part_1(self):
        rule, _, nearby_tickets = interpret_data(test_input)

        self.assertEqual(part_1(rule, nearby_tickets), 71)

    def test_part_2(self):
        rule, my_ticket, nearby_tickets = interpret_data(test_input2)
        part_1(rule, nearby_tickets)
        part_2(rule, my_ticket)
        self.assertEqual(rule.rule_index.get('row'), 0)
        self.assertEqual(rule.rule_index.get('class'), 1)
        self.assertEqual(rule.rule_index.get('seat'), 2)

    def test_part_2_2(self):
        rule, my_ticket, nearby_tickets = interpret_data(test_input)
        part_1(rule, nearby_tickets)
        part_2(rule, my_ticket)
        self.assertEqual(rule.rule_index.get('row'), 0)
        self.assertEqual(rule.rule_index.get('class'), 1)
        self.assertEqual(rule.rule_index.get('seat'), 2)


if __name__ == '__main__':
    print('Day 16')
    data: str = read_input('../inputs/day16.txt')
    rule, my_ticket, nearby_tickets = interpret_data(data)

    print('\tPart 1: {}'.format(part_1(rule, nearby_tickets)))
    print('\tPart 2: {}'.format(part_2(rule, my_ticket)))
    unittest.main()
