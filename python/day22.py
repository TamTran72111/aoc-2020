import unittest
from utilities import read_blocks
from collections import defaultdict, deque
from typing import List, Dict, Set, Optional


class Player:
    def __init__(self, deck: List[int]):
        self.deck = deque(deck)

    def is_out_of_card(self):
        return len(self.deck) == 0

    def draw(self) -> Optional[int]:
        if self.is_out_of_card():
            return None
        return self.deck.popleft()

    def push(self, card: int):
        self.deck.append(card)

    def combat(self, other) -> bool:
        while not self.is_out_of_card() and not other.is_out_of_card():
            my_card = self.draw()
            other_card = other.draw()
            if my_card > other_card:
                self.push(my_card)
                self.push(other_card)
            else:
                other.push(other_card)
                other.push(my_card)

    @property
    def score(self) -> int:
        return sum([(i+1) * card for i, card in enumerate(reversed(self.deck))])

    def is_ready_for_subgame(self, num) -> bool:
        return len(self.deck) >= num

    def prepare_subgame(self, num: int):
        return Player(list(self.deck)[:num])

    def __str__(self) -> str:
        return ','.join([str(card) for card in self.deck])

    def recursive_combat(self, other) -> bool:
        memory = set()
        while not self.is_out_of_card() and not other.is_out_of_card():
            s1 = str(self)
            s2 = str(other)
            if (s1, s2) in memory:
                return True
            memory.add((s1, s2))
            my_card = self.draw()
            other_card = other.draw()
            if self.is_ready_for_subgame(my_card) and other.is_ready_for_subgame(other_card):
                sub_self = self.prepare_subgame(my_card)
                sub_other = other.prepare_subgame(other_card)
                if sub_self.recursive_combat(sub_other):
                    self.push(my_card)
                    self.push(other_card)
                else:
                    other.push(other_card)
                    other.push(my_card)
            elif my_card > other_card:
                self.push(my_card)
                self.push(other_card)
            else:
                other.push(other_card)
                other.push(my_card)
        return other.is_out_of_card()


def solve_part_1(players: List[str]) -> int:
    player1 = Player([int(card) for card in players[0].splitlines()[1:]])
    player2 = Player([int(card) for card in players[1].splitlines()[1:]])
    player1.combat(player2)
    return player1.score + player2.score


def solve_part_2(players: List[str]) -> int:
    player1 = Player([int(card) for card in players[0].splitlines()[1:]])
    player2 = Player([int(card) for card in players[1].splitlines()[1:]])
    if player1.recursive_combat(player2):
        return player1.score

    return player2.score


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

        self.assertEqual(solve_part_1(test_input.split('\n\n')), 306)
        self.assertEqual(solve_part_2(test_input.split('\n\n')), 291)


if __name__ == '__main__':
    print('Day 22')
    import time
    start = time.time()
    players: List[str] = read_blocks('../inputs/day22.txt')

    # solution = Solution(foods)
    print('\tPart 1: {}'.format(solve_part_1(players)))
    print('\tPart 2: {}'.format(solve_part_2(players)))

    print("\tTime: {:.4f}ms".format((time.time() - start) * 1000))
    unittest.main()
