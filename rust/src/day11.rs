use crate::utilities::read_lines;
use std::mem::swap;

fn is_valid_position(layout: &Vec<Vec<char>>, row: isize, col: isize) -> bool {
    row >= 0 && col >= 0 && row < layout.len() as isize && col < layout[0].len() as isize
}

fn is_direction_occupied(
    layout: &Vec<Vec<char>>,
    row: isize,
    col: isize,
    direction: (isize, isize),
    first_visible: bool,
) -> bool {
    let row = row + direction.0;
    let col = col + direction.1;
    if is_valid_position(layout, row, col) {
        if layout[row as usize][col as usize] == '#' {
            return true;
        }
        if layout[row as usize][col as usize] == '.' && first_visible {
            return is_direction_occupied(layout, row, col, direction, first_visible);
        }
    }
    false
}

fn count_occupied_around(
    layout: &Vec<Vec<char>>,
    row: isize,
    col: isize,
    first_visible: bool,
) -> i32 {
    let mut occupied = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for &direction in &directions {
        if is_direction_occupied(layout, row, col, direction, first_visible) {
            occupied += 1;
        }
    }
    occupied
}

fn count_occupied_seats(layout: &Vec<Vec<char>>, first_visible: bool, rule: i32) -> usize {
    let mut new_layout: Vec<Vec<char>> = layout.iter().map(|l| l.clone()).collect();
    let mut layout: Vec<Vec<char>> = layout.iter().map(|l| l.clone()).collect();

    let mut changed = true;
    while changed {
        changed = false;
        for row in 0..layout.len() {
            for col in 0..layout[0].len() {
                if layout[row][col] != '.' {
                    let occupied =
                        count_occupied_around(&layout, row as isize, col as isize, first_visible);
                    if layout[row][col] == 'L' && occupied == 0 {
                        new_layout[row][col] = '#';
                        changed = true;
                    } else if layout[row][col] == '#' && occupied >= rule {
                        new_layout[row][col] = 'L';
                        changed = true;
                    } else {
                        new_layout[row][col] = layout[row][col];
                    }
                }
            }
        }
        swap(&mut layout, &mut new_layout);
    }
    layout
        .into_iter()
        .map(|row| row.into_iter().filter(|c| *c == '#').count())
        .sum()
}

fn part_1(layout: &Vec<Vec<char>>) {
    println!("\tPart 1: {}", count_occupied_seats(layout, false, 4));
}

fn part_2(layout: &Vec<Vec<char>>) {
    println!("\tPart 2: {}", count_occupied_seats(layout, true, 5));
}

pub fn main() {
    println!("Day 11");
    let layout = read_lines("../inputs/day11.txt")
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();
    part_1(&layout);
    part_2(&layout);
}
