use crate::utilities;
use std::collections::HashSet;

pub fn two_sum(data: &[i32], target: i32) -> Option<i32> {
    let mut set = HashSet::new();
    for &number in data {
        let other = target - number;
        match set.get(&other) {
            None => {
                set.insert(number);
            }
            Some(_) => {
                return Some(number * other);
            }
        }
    }
    None
}

fn part_1(data: &[i32]) {
    println!("\tPart 1: {}", two_sum(data, 2020).unwrap());
}

fn part_2(data: &[i32]) {
    let mut result = 0;
    for (index, &number) in data.iter().enumerate() {
        if let Some(other) = two_sum(&data[index + 1..], 2020 - number) {
            result = other * number;
            break;
        }
    }
    if result != 0 {
        println!("\tPart 2: {}", result);
    }
}

pub fn main() {
    println!("Day 1");
    let data = utilities::read_int_array("../inputs/day1.txt");
    part_1(&data);
    part_2(&data);
}
