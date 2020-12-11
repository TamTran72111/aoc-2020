use crate::utilities::read_lines;

fn string_to_int(s: &str) -> i32 {
    let mut ans = 0;
    for ch in s.chars() {
        ans = ans << 1;
        if ch == 'B' || ch == 'R' {
            ans += 1;
        }
    }
    ans
}

fn convert_id(s: String) -> i32 {
    let s_ = s.as_str();
    string_to_int(&s_[..7]) * 8 + string_to_int(&s_[7..])
}

fn part_1(ids: &Vec<i32>) {
    println!("\tPart 1: {}", ids[ids.len() - 1]);
}

fn part_2(ids: &Vec<i32>) {
    println!(
        "\tPart 2: {}",
        ids.windows(2).find(|pair| pair[0] + 1 != pair[1]).unwrap()[0] + 1
    );
}

pub fn main() {
    println!("Day 5");
    let mut ids: Vec<i32> = read_lines("../inputs/day5.txt")
        .into_iter()
        .map(convert_id)
        .collect();
    ids.sort();
    part_1(&ids);
    part_2(&ids);
}
