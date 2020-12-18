use crate::utilities::read_block;
use std::collections::HashMap;

struct FieldRule {
    first_range: (i64, i64),
    second_range: (i64, i64),
}

impl FieldRule {
    fn new(rule: &str) -> Self {
        let mut ranges = rule.split(" or ");
        Self {
            first_range: Self::read_range(ranges.next().unwrap()),
            second_range: Self::read_range(ranges.next().unwrap()),
        }
    }

    fn read_range(range: &str) -> (i64, i64) {
        let mut numbers = range.split('-');
        let low = numbers.next().unwrap().parse().unwrap();
        let high = numbers.next().unwrap().parse().unwrap();
        (low, high)
    }

    fn validate(&self, value: i64) -> bool {
        (self.first_range.0 <= value && self.first_range.1 >= value)
            || (self.second_range.0 <= value && self.second_range.1 >= value)
    }

    fn validate_column(&self, tickets: &Vec<Vec<i64>>, index: usize) -> bool {
        tickets.iter().all(|ticket| self.validate(ticket[index]))
    }
}

struct TicketRule {
    fields: HashMap<String, FieldRule>,
    valid_tickets: Vec<Vec<i64>>,
    field_index: HashMap<String, usize>,
}

impl TicketRule {
    pub fn new(rules: &String) -> Self {
        let mut fields = HashMap::new();
        for rule in rules.split('\n') {
            let mut iter = rule.split(": ");
            let field = iter.next().unwrap().to_string();
            fields.insert(field, FieldRule::new(iter.next().unwrap()));
        }
        Self {
            fields,
            field_index: HashMap::new(),
            valid_tickets: Vec::new(),
        }
    }

    fn validate_value(&self, value: i64) -> bool {
        self.fields.values().any(|field| field.validate(value))
    }

    pub fn sum_invalid(&mut self, nearby_tickets: &String) -> i64 {
        let tickets: Vec<Vec<i64>> = nearby_tickets
            .split('\n')
            .skip(1)
            .map(|line| {
                line.split(',')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect();
        let mut ans = 0;
        for ticket in tickets {
            let mut valid = true;
            for &value in &ticket {
                if !self.validate_value(value) {
                    valid = false;
                    ans += value;
                }
            }
            if valid {
                self.valid_tickets.push(ticket);
            }
        }
        ans
    }

    fn calculate_ticket_product(&mut self, my_ticket: &String) -> i64 {
        let my_ticket: Vec<i64> = my_ticket
            .split('\n')
            .skip(1)
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let mut field_possible_indices = HashMap::new();
        for (field, rule) in &self.fields {
            for index in 0..my_ticket.len() {
                if rule.validate_column(&self.valid_tickets, index) {
                    field_possible_indices
                        .entry(field.clone())
                        .or_insert(vec![])
                        .push(index);
                }
            }
        }
        let mut fields_by_len: Vec<(usize, String)> = field_possible_indices
            .iter()
            .map(|(field, possible_choices)| (possible_choices.len(), field.clone()))
            .collect();

        fields_by_len.sort();

        let mut available_indices = vec![true; my_ticket.len()];

        self.validate_index(
            &fields_by_len,
            &field_possible_indices,
            &mut available_indices,
        );

        self.field_index
            .iter()
            .filter(|(field, _)| field.starts_with("departure"))
            .map(|(_, &index)| my_ticket[index])
            .fold(1, |acc, x| acc * x)
    }

    fn validate_index(
        &mut self,
        fields_by_len: &[(usize, String)],
        field_possible_indices: &HashMap<String, Vec<usize>>,
        available_indices: &mut Vec<bool>,
    ) -> bool {
        if fields_by_len.is_empty() {
            true
        } else {
            let field = &fields_by_len[0].1;
            for &index in field_possible_indices.get(field).unwrap() {
                if available_indices[index] {
                    available_indices[index] = false;
                    if self.validate_index(
                        &fields_by_len[1..],
                        field_possible_indices,
                        available_indices,
                    ) {
                        self.field_index.insert(field.to_string(), index);
                        return true;
                    }
                    available_indices[index] = true;
                }
            }
            false
        }
    }
}

pub fn main() {
    println!("Day 16");
    let data = read_block("../inputs/day16.txt");
    let mut ticket_rule = TicketRule::new(&data[0]);
    println!("\tPart 1: {}", ticket_rule.sum_invalid(&data[2]));
    println!(
        "\tPart 2: {}",
        ticket_rule.calculate_ticket_product(&data[1])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_range() {
        let rule = FieldRule::new("1-3 or 5-7");
        assert!(rule.validate(1));
        assert!(rule.validate(2));
        assert!(rule.validate(3));
        assert!(rule.validate(5));
        assert!(rule.validate(6));
        assert!(rule.validate(7));

        assert!(!rule.validate(0));
        assert!(!rule.validate(4));
        assert!(!rule.validate(8));
        assert!(!rule.validate(100));
    }

    #[test]
    fn test_sum_invalid() {
        let data = vec![
            "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50".to_string(),
            "your ticket:\n7,1,14".to_string(),
            "nearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12".to_string(),
        ];

        let mut ticket_rule = TicketRule::new(&data[0]);
        assert_eq!(ticket_rule.sum_invalid(&data[2]), 71);
    }
}
