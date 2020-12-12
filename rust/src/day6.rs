use crate::utilities::read_block;

struct Group {
    num_of_people: usize,
    answers: Vec<usize>,
}

impl Group {
    fn new(group: String) -> Group {
        let group_answers: Vec<&str> = group.split('\n').collect();
        let num_of_people = group_answers.len();
        let mut answers = vec![0; 26];
        for person_answer in group_answers {
            for answer in person_answer.chars() {
                answers[answer as usize - 97] += 1;
            }
        }
        Group {
            num_of_people,
            answers,
        }
    }

    fn get_total_yes_answers(&self) -> usize {
        self.answers.iter().filter(|answer| **answer > 0).count()
    }

    fn get_everyone_yes(&self) -> usize {
        self.answers
            .iter()
            .filter(|answer| **answer == self.num_of_people)
            .count()
    }
}

fn part_1(groups: &Vec<Group>) {
    println!(
        "\tPart 1: {}",
        groups
            .iter()
            .map(|group| group.get_total_yes_answers())
            .sum::<usize>()
    );
}

fn part_2(groups: &Vec<Group>) {
    println!(
        "\tPart 2: {}",
        groups
            .iter()
            .map(|group| group.get_everyone_yes())
            .sum::<usize>()
    );
}

pub fn main() {
    println!("Day 6");
    let groups = read_block("../inputs/day6.txt")
        .into_iter()
        .map(Group::new)
        .collect();
    part_1(&groups);
    part_2(&groups);
}
