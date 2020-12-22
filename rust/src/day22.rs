use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use crate::utilities::read_block;

#[derive(Debug, Clone)]
struct Player {
    deck: VecDeque<usize>,
}

impl Player {
    fn new(player: &String) -> Self {
        Self {
            deck: player
                .lines()
                .skip(1)
                .map(|card| card.parse().unwrap())
                .collect(),
        }
    }

    fn combat(&mut self, other: &mut Player) -> bool {
        while !(self.is_lost() || other.is_lost()) {
            let self_card = self.draw().unwrap();
            let other_card = other.draw().unwrap();
            let self_win = self_card > other_card;
            self.collect_cards(other, self_win, self_card, other_card);
        }
        other.is_lost()
    }

    fn recursive_combat(&mut self, other: &mut Player) -> bool {
        let mut memory = HashSet::new();
        while !(self.is_lost() || other.is_lost()) {
            let mut hasher = DefaultHasher::new();

            (&self.deck, &other.deck).hash(&mut hasher);
            let game_state = hasher.finish();

            if !memory.insert(game_state) {
                // Found the same game state
                return true;
            }
            let self_card = self.draw().unwrap();
            let other_card = other.draw().unwrap();

            let self_win = if self.can_play_subgame(self_card) && other.can_play_subgame(other_card)
            {
                let mut sub_self = self.prepare_for_subgame(self_card);
                let mut sub_other = other.prepare_for_subgame(other_card);
                sub_self.recursive_combat(&mut sub_other)
            } else {
                self_card > other_card
            };
            self.collect_cards(other, self_win, self_card, other_card);
        }
        other.is_lost()
    }

    fn collect_cards(
        &mut self,
        other: &mut Player,
        self_win: bool,
        self_card: usize,
        other_card: usize,
    ) {
        if self_win {
            self.collect(self_card);
            self.collect(other_card);
        } else {
            other.collect(other_card);
            other.collect(self_card);
        }
    }

    fn can_play_subgame(&self, num: usize) -> bool {
        self.deck.len() >= num
    }

    fn prepare_for_subgame(&self, num: usize) -> Player {
        Player {
            deck: self.deck.iter().take(num).cloned().collect(),
        }
    }

    fn is_lost(&self) -> bool {
        self.deck.is_empty()
    }

    fn draw(&mut self) -> Option<usize> {
        self.deck.pop_front()
    }

    fn collect(&mut self, card: usize) {
        self.deck.push_back(card);
    }

    fn score(&self) -> usize {
        self.deck
            .iter()
            .rev()
            .enumerate()
            .map(|(i, card)| *card * (i + 1))
            .sum()
    }
}

fn solve_part_1(players: &Vec<String>) -> usize {
    let mut player1 = Player::new(&players[0]);
    let mut player2 = Player::new(&players[1]);
    if player1.combat(&mut player2) {
        player1.score()
    } else {
        player2.score()
    }
}

fn solve_part_2(players: &Vec<String>) -> usize {
    let mut player1 = Player::new(&players[0]);
    let mut player2 = Player::new(&players[1]);
    if player1.recursive_combat(&mut player2) {
        player1.score()
    } else {
        player2.score()
    }
}

pub fn main() {
    println!("Day 22");
    let players = read_block("../inputs/day22.txt");
    println!("\tPart1: {}", solve_part_1(&players));
    println!("\tPart2: {}", solve_part_2(&players));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_input = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;
        let players: Vec<String> = test_input.split("\n\n").map(|s| s.to_string()).collect();
        assert_eq!(solve_part_1(&players), 306);
        assert_eq!(solve_part_2(&players), 291);
    }
}
