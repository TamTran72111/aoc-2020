use crate::utilities::read_lines;

struct Policy {
    letter: char,
    first: usize,
    second: usize,
    password: Vec<char>,
}

impl Policy {
    fn new(s: &String) -> Policy {
        let parts: Vec<&str> = s.split(':').collect();
        let policy: Vec<&str> = parts[0].split_ascii_whitespace().collect();
        let letter = policy[1].chars().next().unwrap();
        let numbers: Vec<usize> = policy[0]
            .split('-')
            .map(|num| num.parse().unwrap())
            .collect();
        Policy {
            letter,
            first: numbers[0],
            second: numbers[1],
            password: parts[1].chars().collect(),
        }
    }

    fn validate_password_1(&self) -> bool {
        let mut counter = 0;
        for &ch in &self.password {
            if ch == self.letter {
                counter += 1;
            }
        }
        counter >= self.first && counter <= self.second
    }

    fn validate_password_2(&self) -> bool {
        self.password.len() > self.second
            && self.password[self.first] != self.password[self.second]
            && (self.password[self.first] == self.letter
                || self.password[self.second] == self.letter)
    }
}

fn part_1(data: &Vec<Policy>) {
    println!(
        "\tPart 1: {}",
        data.iter()
            .filter(|policy| policy.validate_password_1())
            .count()
    );
}

fn part_2(data: &Vec<Policy>) {
    println!(
        "\tPart 2: {}",
        data.iter()
            .filter(|policy| policy.validate_password_2())
            .count()
    );
}

pub fn main() {
    println!("Day 2");
    let data: Vec<Policy> = read_lines("../inputs/day2.txt")
        .iter()
        .map(Policy::new)
        .collect();
    part_1(&data);
    part_2(&data);
}
