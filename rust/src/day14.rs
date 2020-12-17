use crate::utilities::read_lines;
use std::collections::HashMap;

const SIZE: usize = 36;

#[derive(PartialEq)]
enum Mode {
    Version1,
    Version2,
}

impl Mode {
    pub fn mask_bit(&self, mask: char, bit: char) -> char {
        if *self == Mode::Version1 && mask == 'X' {
            bit
        } else if *self == Mode::Version2 && mask == '0' {
            bit
        } else {
            mask
        }
    }
}
struct DockingProgram {
    memory: HashMap<u64, u64>,
    mask_string: String,
    mode: Mode,
}

impl DockingProgram {
    pub fn new(mode: Mode) -> DockingProgram {
        DockingProgram {
            memory: HashMap::new(),
            mask_string: String::new(),
            mode,
        }
    }

    fn extract_memory_address(s: &str) -> u64 {
        let start_index = s.find('[').unwrap() + 1;
        let end_index = s.len() - 1;
        s[start_index..end_index].parse().unwrap()
    }

    fn mask(&self, value: u64) -> String {
        if self.mask_string.is_empty() {
            value.to_string()
        } else {
            let mut masked_value = vec!['0'; SIZE];
            let binary_form = format!("{:0>36b}", value);
            for (index, (mask, bit)) in self
                .mask_string
                .chars()
                .zip(binary_form.chars())
                .enumerate()
            {
                masked_value[index] = self.mode.mask_bit(mask, bit);
            }
            masked_value.into_iter().collect()
        }
    }

    fn write_for_v2(&mut self, masked_addr: &mut Vec<char>, value: u64) {
        if masked_addr.contains(&'X') {
            for i in 0..SIZE {
                if masked_addr[i] == 'X' {
                    masked_addr[i] = '0';
                    self.write_for_v2(masked_addr, value);
                    masked_addr[i] = '1';
                    self.write_for_v2(masked_addr, value);
                    masked_addr[i] = 'X';
                    break;
                }
            }
        } else {
            let addr = masked_addr.iter().collect::<String>();
            let memory_address = u64::from_str_radix(addr.as_str(), 2).unwrap();
            self.memory.insert(memory_address, value);
        }
    }

    fn write(&mut self, memory_address: u64, value: u64) {
        match self.mode {
            Mode::Version1 => {
                let masked_value = self.mask(value);
                let value = u64::from_str_radix(masked_value.as_str(), 2).unwrap();
                self.memory.insert(memory_address, value);
            }
            Mode::Version2 => {
                let masked_addr = self.mask(memory_address);
                let mut masked_addr: Vec<char> = masked_addr.chars().collect();
                self.write_for_v2(&mut masked_addr, value);
            }
        }
    }

    pub fn execute(&mut self, instruction: &str) {
        if instruction.starts_with("mask") {
            // Truncate the `mask = ` part in the instruction
            self.mask_string = instruction[7..].to_string();
        } else {
            let mut iter = instruction.split(" = ");
            let memory_address = Self::extract_memory_address(iter.next().unwrap());
            let value: u64 = iter.next().unwrap().parse().unwrap();
            self.write(memory_address, value);
        }
    }

    pub fn get_sum_memory(&self) -> u64 {
        self.memory.values().sum()
    }
}

fn part_1(instructions: &Vec<String>) -> u64 {
    let mut program = DockingProgram::new(Mode::Version1);
    for instruction in instructions {
        program.execute(instruction.as_str())
    }
    program.get_sum_memory()
}

fn part_2(instructions: &Vec<String>) -> u64 {
    let mut program = DockingProgram::new(Mode::Version2);
    for instruction in instructions {
        program.execute(instruction.as_str())
    }

    program.get_sum_memory()
}

pub fn main() {
    println!("Day 14");
    let instructions = read_lines("../inputs/day14.txt");

    println!("\tPart 1: {}", part_1(&instructions));
    println!("\tPart 2: {}", part_2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn test_part_1() {
        let instructions = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];
        assert_eq!(part_1(&instructions), 165);
    }

    #[test]
    fn test_part_2() {
        let instructions = vec![
            "mask = 000000000000000000000000000000X1001X".to_string(),
            "mem[42] = 100".to_string(),
            "mask = 00000000000000000000000000000000X0XX".to_string(),
            "mem[26] = 1".to_string(),
        ];
        assert_eq!(part_2(&instructions), 208);
    }
}
