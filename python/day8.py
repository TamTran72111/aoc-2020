from utilities import read_lines
from typing import List


def check_instructions(instructions: List[str]) -> (int, bool):
    executed = [False] * len(instructions)
    curr_instruction_index = 0
    accumulator = 0
    while curr_instruction_index < len(instructions):
        if executed[curr_instruction_index]:
            # infinite loop happen
            return accumulator, False
        executed[curr_instruction_index] = True

        instruction = instructions[curr_instruction_index]
        if 'nop' in instruction:
            curr_instruction_index += 1
        elif 'jmp' in instruction:
            curr_instruction_index += int(instruction[4:])
        else:
            curr_instruction_index += 1
            accumulator += int(instruction[4:])

    # successfully terminated
    return accumulator, True


def try_terminate(instructions: List[str]) -> int:
    for i in range(len(instructions)):
        instruction = instructions[i]
        if 'nop' in instruction:
            instructions[i] = 'jmp' + instruction[3:]
            accumulator, terminated = check_instructions(instructions)
            if terminated:
                return accumulator
            instructions[i] = instruction
        elif 'jmp' in instruction:
            instructions[i] = 'nop' + instruction[3:]
            accumulator, terminated = check_instructions(instructions)
            if terminated:
                return accumulator
            instructions[i] = instruction

    return -1


def part_1(instructions: List[str]) -> int:
    accumulator, _ = check_instructions(instructions)
    return accumulator


def part_2(instructions: List[str]) -> int:
    return try_terminate(instructions)


if __name__ == '__main__':
    print('Day 8')
    instructions = read_lines('../inputs/day8.txt')
    print('\tPart 1: {}'.format(part_1(instructions)))
    print('\tPart 2: {}'.format(part_2(instructions)))
