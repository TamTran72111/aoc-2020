const PRIME: u64 = 20201227;

fn solve_part_1(public_card: u64, public_door: u64) -> u64 {
    let mut value = 1;
    let mut encrypt = 1;
    while value != public_card {
        value = value * 7 % PRIME;
        encrypt = encrypt * public_door % PRIME;
    }

    encrypt
}

pub fn main() {
    println!("Day 25");

    let public_card = 8335663;
    let public_door = 8614349;
    println!("\tPart1: {}", solve_part_1(public_card, public_door));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let public_card = 5764801;
        let public_door = 17807724;
        assert_eq!(solve_part_1(public_card, public_door), 14897079);
    }
}
