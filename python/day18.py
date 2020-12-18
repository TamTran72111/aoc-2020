import unittest
from utilities import read_lines
from typing import List


def operation_calculate(num_stack: List[int], operation: str):
    if operation == '':
        return
    a = num_stack.pop()
    b = num_stack.pop()

    if operation == '+':
        result = a + b
    else:
        result = a * b

    num_stack.append(result)


def math_calculate_with_parentheses(expr: str, advanced: bool) -> int:
    opens: int = 0
    open_start: int = -1

    for i, c in enumerate(expr):
        if c == '(':
            opens += 1
            if open_start == -1:
                open_start = i
        elif c == ')':
            opens -= 1
            if opens == 0:
                # Calculate the expression inside parentheses first
                # then create calculate a new expression with the stuff
                # inside parentheses replaced by its result
                value = math_calculate(expr[open_start+1:i], advanced)
                return math_calculate(expr[:open_start] + str(value) + expr[i+1:], advanced)


def math_calculate_without_parentheses(expr: str, advanced) -> int:
    curr_num: str = ''
    curr_operation: str = ''
    num_stack: List[int] = []
    operation_stack: List[str] = []

    for c in expr:
        if c.isdigit():
            curr_num += c
        else:
            if curr_num != '':
                num_stack.append(int(curr_num))
            curr_num = ''

            if not advanced or curr_operation == '+':
                operation_calculate(num_stack, curr_operation)
            elif advanced:
                operation_stack.append(curr_operation)
            curr_operation = c

    if curr_num != '':
        num_stack.append(int(curr_num))
    operation_stack.append(curr_operation)
    while len(operation_stack) != 0:
        operation_calculate(num_stack, operation_stack.pop())

    return num_stack[0]


def math_calculate(expr: str, advanced: bool = False) -> int:
    expr: str = expr.replace(' ', '')
    if '(' in expr:
        return math_calculate_with_parentheses(expr, advanced)
    else:
        return math_calculate_without_parentheses(expr, advanced)


def part_1(expressions: List[str]) -> int:
    return sum(map(math_calculate, expressions))


def part_2(expressions: List[str]) -> int:
    return sum([math_calculate(expr, True) for expr in expressions])


class Testing(unittest.TestCase):

    def test_math_calculate(self):
        self.assertEqual(math_calculate("1 + 2 * 3 + 4 * 5 + 6"), 71)
        self.assertEqual(math_calculate("1 + (2 * 3) + (4 * (5 + 6))"), 51)
        self.assertEqual(math_calculate("2 * 3 + (4 * 5)"), 26)
        self.assertEqual(math_calculate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437)
        self.assertEqual(math_calculate(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240)
        self.assertEqual(math_calculate(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632)

    def test_math_calculate_advanced(self):
        self.assertEqual(math_calculate(
            "1 + 2 * 3 + 4 * 5 + 6", True), 231)
        self.assertEqual(math_calculate(
            "1 + (2 * 3) + (4 * (5 + 6))", True), 51)
        self.assertEqual(math_calculate("2 * 3 + (4 * 5)", True), 46)
        self.assertEqual(math_calculate(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)", True), 1445)
        self.assertEqual(math_calculate(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", True), 669060)
        self.assertEqual(math_calculate(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", True), 23340)
        self.assertEqual(math_calculate(
            "3 + (2 * 2 + (7 * 3) * 2) + 7 + 4 + (2 + 6 * 4 + 9 * 4 * 5)", True), 2186)


if __name__ == '__main__':
    print('Day 18')
    expressions: List[str] = read_lines('../inputs/day18.txt')

    print('\tPart 1: {}'.format(part_1(expressions)))
    print('\tPart 2: {}'.format(part_2(expressions)))
    unittest.main()
