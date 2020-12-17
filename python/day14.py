from utilities import read_lines
import unittest
import sys
from typing import List

SIZE = 36


class DockingProgram:
    def __init__(self):
        self.mask_string = None
        self.memory = {}

    def mask_bit(self, mask: str, bit: str) -> str:
        if mask == 'X':
            return bit
        return mask

    def mask(self, value: int) -> str:
        if self.mask_string is None:
            return value
        masked_value = ''
        value = bin(value)[2:]
        if len(value) < SIZE:
            value = '0' * (SIZE - len(value)) + value

        for mask, bit in zip(self.mask_string, value):
            masked_value += self.mask_bit(mask, bit)
        return masked_value

    def write(self, mem_addr: int, value: int):
        masked_value = self.mask(value)
        self.memory[mem_addr] = int(masked_value, 2)

    def __call__(self, instruction: str):
        if instruction.startswith('mask'):
            # Ignore the `mask = ` part in the instruction
            self.mask_string = instruction[7:]
        else:
            parts = instruction.split(' = ')
            start_index = parts[0].find('[') + 1
            mem_addr = int(parts[0][start_index:-1])
            value = int(parts[1])
            self.write(mem_addr, value)

    @property
    def sum_memory(self) -> int:
        return sum(self.memory.values())


class DockingProgramV2(DockingProgram):
    def mask_bit(self, mask: str, bit: str) -> str:
        if mask == '0':
            return bit
        return mask

    def write(self, mem_addr: int, value: int):
        masked_addr = self.mask(mem_addr)

        def helper(masked_addr: str):
            if 'X' not in masked_addr:
                addr = int(masked_addr, 2)
                self.memory[addr] = value
            else:
                index = masked_addr.find('X')
                helper(masked_addr[:index] + '1' + masked_addr[index+1:])
                helper(masked_addr[:index] + '0' + masked_addr[index+1:])
        helper(masked_addr)


def part_1(instructions: List[str]) -> int:
    program = DockingProgram()
    for instruction in instructions:
        program(instruction)

    return program.sum_memory


def part_2(instructions: List[str]) -> int:
    program = DockingProgramV2()
    for instruction in instructions:
        program(instruction)

    return program.sum_memory


class Testing(unittest.TestCase):

    def test_part_1(self):
        instructions = [
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0"
        ]
        self.assertEqual(part_1(instructions), 165)

    def test_part_2(self):
        instructions = [
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1"
        ]
        self.assertEqual(part_2(instructions), 208)


if __name__ == '__main__':
    print('Day 14')
    instructions = read_lines('../inputs/day14.txt')
    print('\tPart 1: {}'.format(part_1(instructions)))
    print('\tPart 2: {}'.format(part_2(instructions)))
    unittest.main()
