use crate::utilities::read_lines;
use std::mem::replace;

fn execute_instructions(instructions: &Vec<String>) -> (i32, bool) {
    let mut executed = vec![false; instructions.len()];
    let mut current = 0;
    let mut accumulator = 0;
    while current < instructions.len() {
        if executed[current] {
            return (accumulator, false);
        }
        executed[current] = true;

        if instructions[current].contains("nop") {
            current += 1;
        } else if instructions[current].contains("jmp") {
            let value = instructions[current].as_str()[5..]
                .parse::<usize>()
                .unwrap();
            if instructions[current].contains('+') {
                current += value;
            } else {
                current -= value;
            }
        } else {
            accumulator += instructions[current].as_str()[4..].parse::<i32>().unwrap();
            current += 1;
        }
    }
    (accumulator, true)
}

fn change_instruction(instructions: &mut Vec<String>) -> i32 {
    let n = instructions.len();
    let instr_changes = [("jmp", "nop"), ("nop", "jmp")];
    for i in 0..n {
        for instr_change in &instr_changes {
            if instructions[i].contains(instr_change.0) {
                let mut new_instruction = instr_change.1.to_string();
                new_instruction.push_str(&instructions[i].as_str()[3..]);
                let old_instruction = replace(&mut instructions[i], new_instruction);
                let (accumulator, terminated) = execute_instructions(instructions);
                let _ = replace(&mut instructions[i], old_instruction);
                if terminated {
                    return accumulator;
                }
            }
        }
    }
    -1
}

fn part_1(instructions: &Vec<String>) {
    let (accumulator, _) = execute_instructions(instructions);
    println!("\tPart 1: {}", accumulator);
}

fn part_2(instructions: &mut Vec<String>) {
    println!("\tPart 2: {}", change_instruction(instructions));
}

pub fn main() {
    println!("Day 8");
    let mut instructions = read_lines("../inputs/day8.txt");
    part_1(&instructions);
    part_2(&mut instructions);
}
