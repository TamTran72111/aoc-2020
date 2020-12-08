from utilities import read_lines
from collections import Counter


class Policy:
    def __init__(self, record):
        parts = record.split(':')
        self.setup_policy(parts[0])
        self.password = parts[1]

    def setup_policy(self, policy):
        parts = policy.split(' ')
        self.letter = parts[1]
        range_ = parts[0].split('-')
        self.low_limit = int(range_[0])
        self.high_limit = int(range_[1])


def validator_1(policy):
    counter = Counter(policy.password)
    return policy.low_limit <= counter[policy.letter] <= policy.high_limit


def validator_2(policy):
    return (policy.password[policy.low_limit] == policy.letter or
            policy.password[policy.high_limit] == policy.letter) and (
        policy.password[policy.low_limit] != policy.password[policy.high_limit]
    )


def validate_policy(line, validator):
    return validator(Policy(line))


def part_1(data):
    return sum(map(lambda line: validate_policy(line, validator_1), data))


def part_2(data):
    return sum(map(lambda line: validate_policy(line, validator_2), data))


if __name__ == '__main__':
    print('Day 2')
    data = read_lines('../inputs/day2.txt')
    print('\tPart 1: {}'.format(part_1(data)))
    print('\tPart 2: {}'.format(part_2(data)))
