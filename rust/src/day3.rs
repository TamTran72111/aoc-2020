use crate::utilities::read_lines;

fn count_trees(data: &Vec<Vec<char>>, move_: &(usize, usize)) -> i32 {
    let mut trees = 0;
    let mut row = 0;
    let mut col = 0;

    while row < data.len() {
        if data[row][col] == '#' {
            trees += 1;
        }
        row += move_.1;
        col += move_.0;
        col %= data[0].len();
    }

    trees
}

fn part_1(data: &Vec<Vec<char>>) {
    let move_ = (3, 1);
    println!("\tPart 1: {}", count_trees(data, &move_));
}

fn part_2(data: &Vec<Vec<char>>) {
    let moves = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!(
        "\tPart 2: {}",
        moves
            .iter()
            .map(|move_| count_trees(data, move_))
            .fold(1, |x, y| x * y)
    );
}

pub fn main() {
    println!("Day 3");
    let data: Vec<Vec<char>> = read_lines("../inputs/day3.txt")
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    part_1(&data);
    part_2(&data);
}
