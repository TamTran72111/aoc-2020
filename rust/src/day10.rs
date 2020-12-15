use crate::utilities::read_int_array;

fn find_differences_product(adapters: &Vec<i64>) -> i64 {
    let mut prev = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 1;
    for &adapter in adapters {
        if adapter - prev == 1 {
            diff_1 += 1;
        } else if adapter - prev == 3 {
            diff_3 += 1;
        }
        prev = adapter;
    }
    diff_1 * diff_3
}

fn find_num_of_arrangments(adapters: &Vec<i64>) -> i64 {
    let mut arrangements = vec![0; adapters.len()];
    for i in 0..3 {
        if adapters[i] <= 3 {
            arrangements[i] = 1;
        }
    }
    for i in 1..adapters.len() {
        for j in 1..4 {
            if i >= j && adapters[i] - adapters[i - j] <= 3 {
                arrangements[i] += arrangements[i - j];
            }
        }
    }
    arrangements.pop().unwrap()
}

fn part_1(adapters: &Vec<i64>) {
    println!("\tPart 1: {}", find_differences_product(adapters));
}

fn part_2(adapters: &Vec<i64>) {
    println!("\tPart 2: {}", find_num_of_arrangments(adapters));
}

pub fn main() {
    println!("Day 10");
    let mut adapters = read_int_array("../inputs/day10.txt");
    adapters.sort();
    part_1(&adapters);
    part_2(&adapters);
}
