use std::collections::HashMap;

fn memory_game(starting_numbers: &[usize], last_turn: usize) -> usize {
    let mut last_num = *starting_numbers.last().unwrap();
    let mut memory = vec![-1isize; 30_000_000];
    for i in 0..starting_numbers.len() - 1 {
        memory[starting_numbers[i]] = i as isize;
    }
    for turn in starting_numbers.len() - 1..last_turn - 1 {
        if memory[last_num] == -1 {
            memory[last_num] = turn as isize;
            last_num = 0;
        } else {
            let new_last_num = turn - memory[last_num] as usize;
            memory[last_num] = turn as isize;
            last_num = new_last_num;
        }
    }
    last_num
}

pub fn main() {
    println!("Day 15");
    let starting_numbers = [1, 20, 8, 12, 0, 14];
    println!("\tPart 1: {}", memory_game(&starting_numbers, 2020));
    println!("\tPart 2: {}", memory_game(&starting_numbers, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_turn() {
        let starting_numbers = [0, 3, 6];
        assert_eq!(memory_game(&starting_numbers, 4), 0);
        assert_eq!(memory_game(&starting_numbers, 5), 3);
        assert_eq!(memory_game(&starting_numbers, 6), 3);
        assert_eq!(memory_game(&starting_numbers, 7), 1);
        assert_eq!(memory_game(&starting_numbers, 8), 0);
        assert_eq!(memory_game(&starting_numbers, 9), 4);
        assert_eq!(memory_game(&starting_numbers, 10), 0);
    }

    #[test]
    fn test_2020() {
        assert_eq!(memory_game(&[0, 3, 6], 2020), 436);
        assert_eq!(memory_game(&[1, 3, 2], 2020), 1);
        assert_eq!(memory_game(&[2, 1, 3], 2020), 10);
        assert_eq!(memory_game(&[1, 2, 3], 2020), 27);
        assert_eq!(memory_game(&[2, 3, 1], 2020), 78);
        assert_eq!(memory_game(&[3, 2, 1], 2020), 438);
        assert_eq!(memory_game(&[3, 1, 2], 2020), 1836);
    }

    #[test]
    fn test_large_turn() {
        assert_eq!(memory_game(&[0, 3, 6], 30000000), 175594);
        assert_eq!(memory_game(&[1, 3, 2], 30000000), 2578);
        assert_eq!(memory_game(&[2, 1, 3], 30000000), 3544142);
        assert_eq!(memory_game(&[1, 2, 3], 30000000), 261214);
        assert_eq!(memory_game(&[2, 3, 1], 30000000), 6895259);
        assert_eq!(memory_game(&[3, 2, 1], 30000000), 18);
        assert_eq!(memory_game(&[3, 1, 2], 30000000), 362);
    }
}
