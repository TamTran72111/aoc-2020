import unittest
from utilities import read_blocks
from collections import defaultdict, deque
from typing import List, Dict, Set, Optional


def score(deck: List[int]) -> int:
    return sum([(i+1) * card for i, card in enumerate(reversed(deck))])


def solve_part_1(deck1: List[int], deck2: List[int]) -> int:
    while len(deck1) and len(deck2):
        card1 = deck1[0]
        card2 = deck2[0]
        if card1 > card2:
            deck1 = deck1[1:] + [card1, card2]
            deck2 = deck2[1:]
        else:
            deck1 = deck1[1:]
            deck2 = deck2[1:] + [card2, card1]
    return score(deck1) if len(deck1) else score(deck2)


def recursive_combat(deck1: List[int], deck2: List[int]) -> bool:
    memory = set()
    while len(deck1) and len(deck2):
        s = (tuple(deck1), tuple(deck2))
        if s in memory:
            return True, deck1
        memory.add(s)

        card1 = deck1[0]
        card2 = deck2[0]

        if len(deck1) > card1 and len(deck2) > card2:
            win, _ = recursive_combat(deck1[1:card1+1], deck2[1:card2+1])
        else:
            win = card1 > card2

        if win:
            deck1 = deck1[1:] + [card1, card2]
            deck2 = deck2[1:]
        else:
            deck1 = deck1[1:]
            deck2 = deck2[1:] + [card2, card1]

    if len(deck1):
        return True, deck1
    else:
        return False, deck2


def solve_part_2(deck1: List[int], deck2: List[int]) -> int:
    _, deck = recursive_combat(deck1, deck2)
    return score(deck)


class Testing(unittest.TestCase):

    def test_solution(self):
        test_input = """Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"""
        deck1 = [int(x) for x in test_input.split('\n\n')[0].splitlines()[1:]]
        deck2 = [int(x) for x in test_input.split('\n\n')[1].splitlines()[1:]]

        self.assertEqual(solve_part_1(deck1[:], deck2[:]), 306)
        self.assertEqual(solve_part_2(deck1, deck2), 291)


if __name__ == '__main__':
    print('Day 22')
    import time
    start = time.time()
    players: List[str] = read_blocks('../inputs/day22.txt')

    deck1 = [int(card) for card in players[0].splitlines()[1:]]
    deck2 = [int(card) for card in players[1].splitlines()[1:]]

    print('\tPart 1: {}'.format(solve_part_1(deck1[:], deck2[:])))
    print('\tPart 2: {}'.format(solve_part_2(deck1, deck2)))

    print("\tTime: {:.4f}ms".format((time.time() - start) * 1000))
    unittest.main()
