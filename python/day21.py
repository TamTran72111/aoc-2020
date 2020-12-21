import unittest
from utilities import read_lines
from collections import defaultdict, deque
from typing import List, Dict, Set


class Solution:
    def __init__(self, foods: List[str]):
        self.ingredients: Dict[str, Set[str]] = {}
        self.allergens: Dict[str, Set[str]] = {}
        self.ingredient_counters = defaultdict(int)
        self.total_ingredients = 0
        for food in foods:
            parts = food.split(' (')
            ingredients = set(parts[0].split(' '))
            # from 9 to -1 to ignore the `contains ` word and the `)` at the end
            allergens = set(parts[1][9:-1].split(', '))

            for ingredient in ingredients:
                self.ingredient_counters[ingredient] += 1
                self.total_ingredients += 1
                if ingredient in self.ingredients:
                    self.ingredients[ingredient] = allergens.union(
                        self.ingredients[ingredient]
                    )
                else:
                    self.ingredients[ingredient] = allergens

            for allergen in allergens:
                if allergen in self.allergens:
                    self.allergens[allergen] = ingredients.intersection(
                        self.allergens[allergen]
                    )
                else:
                    self.allergens[allergen] = ingredients

    def __get_queue(self):
        for allergen, ingredients in self.allergens.items():
            if len(ingredients) == 1 and allergen not in self.seen:
                self.queue.append(allergen)
                self.seen.add(allergen)

    def solve_part_1(self):
        self.seen = set()
        self.queue = deque()
        self.__get_queue()
        while len(self.queue) > 0:
            allergen = self.queue.popleft()
            for ingredient in self.allergens[allergen]:
                allergens = list(self.ingredients[ingredient])
                for aller in allergens:
                    if aller != allergen and ingredient in self.allergens[aller]:
                        self.allergens[aller].remove(ingredient)
                        self.ingredients[ingredient].remove(aller)
            self.__get_queue()

        count_valid_ingredients = 0
        for ingredients in self.allergens.values():
            # Set of ingredients for each allergens should contain only one
            # ingredient at this point
            for ingredient in ingredients:
                count_valid_ingredients += self.ingredient_counters[ingredient]

        return self.total_ingredients - count_valid_ingredients

    def solve_part_2(self):
        result = []
        for allergen, ingredients in self.allergens.items():
            for ingredient in ingredients:
                result.append((allergen, ingredient))

        # Order pairs of allergens and ingridents alphabetically
        result.sort()
        return ','.join(map(lambda pair: pair[1], result))


class Testing(unittest.TestCase):

    def test_solution(self):
        test_input = """mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"""

        solution = Solution(test_input.splitlines())
        self.assertEqual(solution.solve_part_1(), 5)
        self.assertEqual(solution.solve_part_2(), 'mxmxvkd,sqjhc,fvjkl')


if __name__ == '__main__':
    print('Day 21')
    import time
    start = time.time()
    foods: List[str] = read_lines('../inputs/day21.txt')

    solution = Solution(foods)
    print('\tPart 1: {}'.format(solution.solve_part_1()))
    print('\tPart 2: {}'.format(solution.solve_part_2()))

    print("\tTime: {:.4f}ms".format((time.time() - start) * 1000))
    unittest.main()
