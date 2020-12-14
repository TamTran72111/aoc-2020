use crate::utilities::read_int_array;
use std::collections::{HashSet, VecDeque};

fn two_sum(preamble: &VecDeque<i64>, target: i64) -> bool {
    let mut set = HashSet::new();
    for &number in preamble {
        let other = target - number;
        if set.contains(&other) {
            return true;
        }
        set.insert(number);
    }
    false
}

fn find_invalid(numbers: &Vec<i64>) -> i64 {
    let mut preamble = VecDeque::new();
    for &number in &numbers[..25] {
        preamble.push_back(number);
    }

    for &number in &numbers[25..] {
        if !two_sum(&preamble, number) {
            return number;
        }
        preamble.push_back(number);
        let _ = preamble.pop_front();
    }
    unreachable!("The correct input will not reach this point")
}

fn find_weakness(numbers: &Vec<i64>, target: i64) -> i64 {
    let mut contiguous_sums = vec![];
    contiguous_sums.push(numbers[0] + numbers[1]);

    for last in 2..numbers.len() {
        contiguous_sums
            .iter_mut()
            .for_each(|sum| *sum += numbers[last]);

        contiguous_sums.push(numbers[last] + numbers[last - 1]);

        for (start, &sum) in contiguous_sums.iter().enumerate() {
            if sum == target {
                return numbers[start..last + 1].iter().min().unwrap()
                    + numbers[start..last + 1].iter().max().unwrap();
            }
        }
    }
    -1
}

fn part_1(numbers: &Vec<i64>) {
    println!("\tPart 1: {}", find_invalid(numbers));
}

fn part_2(numbers: &Vec<i64>) {
    println!(
        "\tPart 2: {}",
        find_weakness(numbers, find_invalid(numbers))
    );
}

pub fn main() {
    println!("Day 9");
    let numbers = read_int_array("../inputs/day9.txt");
    part_1(&numbers);
    part_2(&numbers);
}
