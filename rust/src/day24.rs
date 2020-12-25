use std::collections::{HashMap, HashSet};

use crate::utilities::read_lines;

const DIRECTIONS: [&str; 6] = ["ne", "nw", "se", "sw", "e", "w"];
const CHANGES: [(i16, i16); 6] = [(0, 1), (-1, 1), (1, -1), (0, -1), (1, 0), (-1, 0)];

fn solve_part_1(lines: Vec<String>) -> HashSet<(i16, i16)> {
    let mut black_tiles = HashSet::new();
    for line in lines {
        // Reference tile position
        let mut pos = (0, 0);
        let mut s = line.as_str();
        while !s.is_empty() {
            for (index, direction) in DIRECTIONS.iter().enumerate() {
                if s.starts_with(*direction) {
                    s = &s[direction.len()..];
                    pos.0 += CHANGES[index].0;
                    pos.1 += CHANGES[index].1;
                    break;
                }
            }
        }
        if black_tiles.contains(&pos) {
            black_tiles.remove(&pos);
        } else {
            black_tiles.insert(pos);
        }
    }
    black_tiles
}

fn solve_part_2(mut black_tiles: HashSet<(i16, i16)>) -> usize {
    for _ in 0..100 {
        let mut no_flip_black_tiles = HashSet::new();
        let mut white_tiles = HashMap::new();
        for tile in &black_tiles {
            let mut black_neighbors = 0;
            for change in &CHANGES {
                let neighbor = (tile.0 + change.0, tile.1 + change.1);
                if black_tiles.contains(&neighbor) {
                    black_neighbors += 1;
                } else {
                    // This is a white neighbor, so increase the number of
                    // black neighbors for this white neighbor by 1, because
                    // it's a neighbor to the current black `tile`
                    *white_tiles.entry(neighbor).or_insert(0) += 1;
                }
            }

            if black_neighbors == 1 || black_neighbors == 2 {
                no_flip_black_tiles.insert(*tile);
            }
        }
        let white_to_black: HashSet<(i16, i16)> = white_tiles
            .into_iter()
            .filter(|(_, black_neighbors)| *black_neighbors == 2)
            .map(|(pos, _)| pos)
            .collect();
        black_tiles = no_flip_black_tiles
            .union(&white_to_black)
            .cloned()
            .collect::<HashSet<(i16, i16)>>();
    }
    black_tiles.len()
}
pub fn main() {
    println!("Day 24");
    let lines = read_lines("../inputs/day24.txt");
    let black_tiles = solve_part_1(lines);
    println!("\tPart1: {}", black_tiles.len());
    println!("\tPart2: {}", solve_part_2(black_tiles));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_str = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;
        assert_eq!(
            solve_part_1(input_str.lines().map(str::to_string).collect()).len(),
            10
        );
    }

    #[test]
    fn test_part_2() {
        let input_str = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;
        let black_tiles = solve_part_1(input_str.lines().map(str::to_string).collect());
        assert_eq!(solve_part_2(black_tiles), 2208);
    }
}
