fn perform_move(circle: &mut Vec<u32>, current: u32, wrap_value: u32) {
    // Pick up the 3 cups immediately after the current cup.
    // Using 3 variable instead of
    let mut pickups = [0, 0, 0];
    let mut pickup = circle[current as usize]; // The first cup after the current cup
    for i in 0..3 {
        pickups[i] = pickup;
        pickup = circle[pickup as usize]; // The next cup after the picked up cup
    }
    // At this point `pickup` will be the cup after the 3 picked up cups, so we link
    // the current cup to pickup to continue the circle
    circle[current as usize] = pickup;

    let mut destination = current;
    while destination == current || pickups.contains(&destination) {
        destination -= 1;
        if destination == 0 {
            destination = wrap_value;
        }
    }

    // Insert the 3 picked up cups after the destination, so the last picked up
    // cup will point to the cup after the destination, and the destination
    // will point to the first picked up cup
    circle[pickups[2] as usize] = circle[destination as usize];
    circle[destination as usize] = pickups[0];
}

fn solve_part_1(initial: &str, moves: usize) -> String {
    let nums: Vec<u32> = initial.chars().map(|n| n as u32 - 48).collect();
    let mut circle = vec![0; 10];

    for i in 1..nums.len() {
        // the cup labelled nums[i-1] will point to the cup labelled nums[i]
        circle[nums[i - 1] as usize] = nums[i];
    }

    // The last cup will point back the the first cup to form a circle
    circle[*nums.last().unwrap() as usize] = nums[0];
    let mut current_cup = nums[0];

    for _ in 0..moves {
        perform_move(&mut circle, current_cup, 9);
        // The next current cup is the cup after the current cup
        current_cup = circle[current_cup as usize];
    }

    let mut result = String::with_capacity(8);
    current_cup = circle[1];
    // Get the order of all values except 1
    for _ in 0..8 {
        result.push((48 + current_cup as u8) as char);
        current_cup = circle[current_cup as usize];
    }
    result
}

fn solve_part_2(initial: &str, moves: usize) -> u64 {
    let nums: Vec<u32> = initial.chars().map(|n| n as u32 - 48).collect();
    let mut circle = vec![0; 1_000_001];

    for i in 1..nums.len() {
        // the cup labelled nums[i-1] will point to the cup labelled nums[i]
        circle[nums[i - 1] as usize] = nums[i];
    }
    circle[*nums.last().unwrap() as usize] = 10;

    for i in 10..1_000_000 {
        // The cup is in increasing order
        circle[i] = i as u32 + 1;
    }

    // The last cup will point back the the first cup to form a circle
    circle[1_000_000] = nums[0];

    let mut current_cup = nums[0];

    for _ in 0..moves {
        perform_move(&mut circle, current_cup, 1_000_000);
        // The next current cup is the cup after the current cup
        current_cup = circle[current_cup as usize];
    }

    circle[1] as u64 * circle[circle[1] as usize] as u64
}

pub fn main() {
    println!("Day 23");
    let initial = "538914762";
    println!("\tPart1: {}", solve_part_1(initial, 100));
    println!("\tPart2: {}", solve_part_2(initial, 10_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let initial = "389125467";
        assert_eq!(solve_part_1(initial, 10), "92658374".to_string());
        assert_eq!(solve_part_1(initial, 100), "67384529".to_string());
    }

    #[test]
    fn test_part_2() {
        let initial = "389125467";
        assert_eq!(solve_part_2(initial, 10_000_000), 149245887792);
    }
}
