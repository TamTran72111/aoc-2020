use std::{collections::HashMap, mem::swap};

use crate::utilities::read_block;

enum Rule {
    Match(char),
    RuleList(Vec<Vec<usize>>),
}

impl Rule {
    fn new(rule: &str) -> Rule {
        if rule.contains('"') {
            Rule::Match(rule.chars().skip(1).next().unwrap())
        } else {
            Rule::RuleList(
                rule.split(" | ")
                    .map(|list| {
                        list.split(' ')
                            .map(|r| r.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect(),
            )
        }
    }

    fn match_message(&self, index: usize, message: &Vec<char>, rule_map: &RuleMap) -> Vec<usize> {
        if index == message.len() {
            vec![]
        } else {
            match self {
                Rule::Match(c) => {
                    if *c == message[index] {
                        vec![index + 1]
                    } else {
                        vec![]
                    }
                }
                Rule::RuleList(options) => {
                    let mut result = vec![];
                    for rules in options {
                        let mut start_indices = vec![index];
                        for &rule in rules {
                            let mut new_start_indices = vec![];
                            for &index in &start_indices {
                                new_start_indices
                                    .extend(rule_map.match_message(rule, index, message).iter());
                            }
                            swap(&mut start_indices, &mut new_start_indices);
                        }
                        result.extend(start_indices.iter());
                    }
                    result
                }
            }
        }
    }
}

struct RuleMap {
    rules: HashMap<usize, Rule>,
}

impl RuleMap {
    fn new(rules_str: String) -> Self {
        let mut rules = HashMap::new();
        for rule_str in rules_str.split('\n') {
            let mut iter = rule_str.split(": ");
            let rule = iter.next().unwrap().parse().unwrap();
            rules.insert(rule, Rule::new(iter.next().unwrap()));
        }
        Self { rules }
    }

    fn match_message(&self, rule: usize, index: usize, message: &Vec<char>) -> Vec<usize> {
        self.rules
            .get(&rule)
            .unwrap()
            .match_message(index, message, self)
    }

    fn is_matched(&self, message: &Vec<char>) -> bool {
        let matched_lengths = self.match_message(0, 0, message);
        matched_lengths
            .into_iter()
            .any(|length| length == message.len())
    }
}

fn count_match_message(rule_string: String, messages: &Vec<Vec<char>>) -> usize {
    let rule_map = RuleMap::new(rule_string);
    messages
        .iter()
        .filter(|&message| rule_map.is_matched(message))
        .count()
}

pub fn main() {
    println!("Day 19");
    let mut data = read_block("../inputs/day19.txt");
    let messages = data
        .pop()
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let rule_string = data.pop().unwrap();

    println!(
        "\tPart 1: {}",
        count_match_message(rule_string.clone(), &messages)
    );
    println!(
        "\tPart 2: {}",
        count_match_message(
            rule_string
                .replace("8: 42", "8: 42 | 42 8")
                .replace("11: 42 31", "11: 42 31 | 42 11 31"),
            &messages
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let rule_str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#
            .to_string();
        let messages_str = r#"ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let messages = messages_str
            .split('\n')
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        assert_eq!(count_match_message(rule_str, &messages), 2);
    }

    #[test]
    fn test_part_2() {
        let messages_str = r#"abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let rule_str = r#"042: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"#
            .to_string();
        let messages = messages_str
            .split('\n')
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        assert_eq!(count_match_message(rule_str.clone(), &messages), 3);
        assert_eq!(
            count_match_message(
                rule_str
                    .replace("8: 42", "8: 42 | 42 8")
                    .replace("11: 42 31", "11: 42 31 | 42 11 31"),
                &messages
            ),
            12
        );
    }
}
