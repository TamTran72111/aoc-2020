use crate::utilities::read_lines;

const DIRECTIONS: [char; 4] = ['N', 'E', 'S', 'W'];
const DIRECTION_FACTORS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn calculate_manhattan_distance(instructions: &Vec<String>) -> i32 {
    let (mut x, mut y) = (0i32, 0i32); // Starting position

    let mut direction_index = 1usize; // Initially facing east

    for instruction in instructions {
        let action = instruction.chars().next().unwrap();
        let value: i32 = instruction.as_str()[1..].parse().unwrap();

        if action == 'F' {
            x += value * DIRECTION_FACTORS[direction_index].0;
            y += value * DIRECTION_FACTORS[direction_index].1;
        } else if action == 'R' {
            direction_index += value as usize / 90;
            direction_index %= 4;
        } else if action == 'L' {
            direction_index += 4 - value as usize / 90;
            direction_index %= 4;
        } else {
            for i in 0..4 {
                if DIRECTIONS[i] == action {
                    x += value * DIRECTION_FACTORS[i].0;
                    y += value * DIRECTION_FACTORS[i].1;
                    break;
                }
            }
        }
    }
    x.abs() + y.abs()
}

fn calculate_manhattan_distance_with_waypoint(instructions: &Vec<String>) -> i32 {
    let (mut x, mut y) = (0i32, 0i32); // Starting position
    let (mut waypoint_x, mut waypoint_y) = (10i32, 1i32); // Starting waypoint

    for instruction in instructions {
        let action = instruction.chars().next().unwrap();
        let value: i32 = instruction.as_str()[1..].parse().unwrap();

        if action == 'F' {
            x += value * waypoint_x;
            y += value * waypoint_y;
        } else if action == 'R' || action == 'L' {
            if value == 180 {
                waypoint_x = -waypoint_x;
                waypoint_y = -waypoint_y;
            } else {
                let right_rotation = if action == 'R' { value } else { 360 - value };

                let factor = if right_rotation == 90 {
                    (1, -1)
                } else {
                    (-1, 1)
                };
                let tmp = waypoint_x;
                waypoint_x = factor.0 * waypoint_y;
                waypoint_y = factor.1 * tmp;
            }
        } else {
            for i in 0..4 {
                if DIRECTIONS[i] == action {
                    waypoint_x += value * DIRECTION_FACTORS[i].0;
                    waypoint_y += value * DIRECTION_FACTORS[i].1;
                    break;
                }
            }
        }
    }
    x.abs() + y.abs()
}

fn part_1(instructions: &Vec<String>) {
    println!("\tPart 1: {}", calculate_manhattan_distance(instructions));
}

fn part_2(instructions: &Vec<String>) {
    println!(
        "\tPart 2: {}",
        calculate_manhattan_distance_with_waypoint(instructions)
    );
}

pub fn main() {
    println!("Day 12");
    let instructions = read_lines("../inputs/day12.txt");
    part_1(&instructions);
    part_2(&instructions);
}
