use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use crate::utilities::read_block;
const MONSTER_OFFSETS: [(usize, usize); 15] = [
    (1, 0),
    (2, 1),
    (2, 4),
    (1, 5),
    (1, 6),
    (2, 7),
    (2, 10),
    (1, 11),
    (1, 12),
    (2, 13),
    (2, 16),
    (1, 17),
    (0, 18),
    (1, 18),
    (1, 19),
];
#[derive(Clone, Copy, Debug)]
enum Border {
    Top,
    Left,
    Right,
    Bottom,
}

impl Border {
    fn get_border(&self, tile: &Vec<String>) -> String {
        match self {
            Border::Top => tile[0].clone(),
            Border::Bottom => tile.last().unwrap().clone(),
            Border::Left => tile.iter().map(|s| s.chars().next().unwrap()).collect(),
            Border::Right => tile.iter().map(|s| s.chars().last().unwrap()).collect(),
        }
    }
}
struct Tile {
    tile: Vec<String>,
    borders: HashSet<String>,
    id: u64,
}

impl Tile {
    fn new(tile_info: String) -> Self {
        let mut iter = tile_info.lines();
        let header = iter.next().unwrap();
        let id = header[5..header.len() - 1].parse().unwrap();
        let tile: Vec<String> = iter.map(|line| line.to_string()).collect();
        let mut borders: HashSet<String> = HashSet::new();
        let top = Border::Top.get_border(&tile);
        borders.insert(top.chars().rev().collect());
        borders.insert(top);
        let bottom = Border::Bottom.get_border(&tile);
        borders.insert(bottom.chars().rev().collect());
        borders.insert(bottom);
        let left: String = Border::Left.get_border(&tile);
        borders.insert(left.chars().rev().collect());
        borders.insert(left);
        let right: String = Border::Right.get_border(&tile);
        borders.insert(right.chars().rev().collect());
        borders.insert(right);
        Self { id, tile, borders }
    }

    fn is_neighbor(&self, other: &Tile) -> bool {
        self.id != other.id && self.borders.intersection(&other.borders).count() > 0
    }

    fn without_borders(&self) -> Vec<Vec<char>> {
        let len = self.tile.len();
        self.tile
            .iter()
            .skip(1)
            .map(|line| line.chars().skip(1).take(len - 2).collect::<Vec<char>>())
            .take(len - 2)
            .collect()
    }

    fn match_border(&self, other: &Tile, border: Border) -> bool {
        other.borders.contains(&border.get_border(&self.tile))
    }

    fn flip(&mut self) {
        self.tile = self.tile.iter().rev().map(|s| s.clone()).collect();
    }

    fn rotate(&mut self) {
        let tile: Vec<Vec<char>> = self
            .tile
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        // let mut new_tile = vec![vec!['.'; tile.len()]; tile.len()];
        // for i in 0..tile.len() {
        //     for j in 0..tile.len() {
        //         new_tile[i][j] = tile[j][tile.len() - 1 - i];
        //     }
        // }
        self.tile = rotate(tile)
            .into_iter()
            .map(|s| s.into_iter().collect())
            .collect();
    }

    fn is_top_left(&self, neighbor_1: &Tile, neighbor_2: &Tile) -> bool {
        (self.match_border(neighbor_1, Border::Right)
            && self.match_border(neighbor_2, Border::Bottom))
            || (self.match_border(neighbor_2, Border::Right)
                && self.match_border(neighbor_1, Border::Bottom))
    }

    fn as_top_left(&mut self, neighbor_1: &Tile, neighbor_2: &Tile) -> bool {
        for _ in 0..4 {
            if self.is_top_left(neighbor_1, neighbor_2) {
                return true;
            }
            // Rotate to adapt to the position
            self.rotate();
        }
        self.flip();
        for _ in 0..4 {
            if self.is_top_left(neighbor_1, neighbor_2) {
                return true;
            }
            // Rotate to adapt to the position
            self.rotate();
        }
        false
    }

    fn adapt(&mut self, target_border: String, border: Border) {
        for _ in 0..4 {
            if border.get_border(&self.tile) == target_border {
                return;
            }
            // Rotate to adapt to the position
            self.rotate();
        }
        self.flip();
        for _ in 0..4 {
            if border.get_border(&self.tile) == target_border {
                return;
            }
            // Rotate to adapt to the position
            self.rotate();
        }

        unreachable!()
    }
}

struct Solution {
    tiles: HashMap<u64, RefCell<Tile>>,
    neighbors: HashMap<u64, Vec<u64>>,
    cornors: Vec<u64>,
}

impl Solution {
    fn new(tiles: Vec<String>) -> Self {
        let tiles_vec: Vec<Tile> = tiles.into_iter().map(Tile::new).collect();
        let mut tiles = HashMap::new();
        let mut neighbors: HashMap<u64, Vec<u64>> = HashMap::new();

        for i in 0..tiles_vec.len() {
            for j in 0..tiles_vec.len() {
                if tiles_vec[i].is_neighbor(&tiles_vec[j]) {
                    neighbors
                        .entry(tiles_vec[i].id)
                        .or_default()
                        .push(tiles_vec[j].id);
                }
            }
        }

        for tile in tiles_vec {
            tiles.insert(tile.id, RefCell::new(tile));
        }
        Self {
            tiles,
            neighbors,
            cornors: Vec::new(),
        }
    }

    fn solve_part_1(&mut self) -> u64 {
        let mut answer = 1;
        for (&id, neighbors) in self.neighbors.iter() {
            if neighbors.len() == 2 {
                self.cornors.push(id);
                answer *= id;
            }
        }
        answer
    }

    fn solve_part_2(&mut self) -> usize {
        let image = self.construct_image();
        let total_hash: usize = image
            .iter()
            .map(|row| row.iter().filter(|pixel| **pixel == '#').count())
            .sum();

        total_hash - 15 * count_monsters(image)
    }

    fn construct_image(&mut self) -> Vec<Vec<char>> {
        let mut current_id = self.find_top_left();
        let mut tile_image = vec![];
        while current_id.is_some() {
            let (new_current_id, row) = self.construct_row(current_id);
            tile_image.push(row);
            current_id = new_current_id;
        }

        let mut image = vec![];
        for row in tile_image {
            let tiles_without_border: Vec<Vec<Vec<char>>> = row
                .into_iter()
                .map(|id| self.tiles.get(&id).unwrap().borrow().without_borders())
                .collect();
            let mut tmp = vec![];
            let len = tiles_without_border[0].len();
            for i in 0..len {
                let mut new_row: Vec<char> = vec![];
                for tile in &tiles_without_border {
                    new_row.extend(tile[i].iter());
                }
                tmp.push(new_row);
            }
            image.extend(tmp.into_iter());
        }
        image
    }

    fn find_top_left(&self) -> Option<u64> {
        for id in &self.cornors {
            let mut tile = self.tiles.get(id).unwrap().borrow_mut();
            let neighbors = self.neighbors.get(id).unwrap();
            let neighbor_1 = self.tiles.get(&neighbors[0]).unwrap().borrow();
            let neighbor_2 = self.tiles.get(&neighbors[1]).unwrap().borrow();
            if tile.as_top_left(&neighbor_1, &neighbor_2) {
                return Some(*id);
            }
        }
        None
    }

    fn construct_row(&self, mut current_id: Option<u64>) -> (Option<u64>, Vec<u64>) {
        let mut row = vec![];
        while let Some(id) = current_id {
            row.push(id);
            current_id = self.connect_borders(id, Border::Right, Border::Left);
        }
        // Get id of the first tile of the next row
        current_id = self.connect_borders(row[0], Border::Bottom, Border::Top);
        (current_id, row)
    }

    fn connect_borders(
        &self,
        current_id: u64,
        match_border: Border,
        adapt_border: Border,
    ) -> Option<u64> {
        let tile = self.tiles.get(&current_id).unwrap().borrow();
        for neighbor_id in self.neighbors.get(&current_id).unwrap() {
            let mut neighbor = self.tiles.get(neighbor_id).unwrap().borrow_mut();
            if tile.match_border(&neighbor, match_border) {
                neighbor.adapt(match_border.get_border(&tile.tile), adapt_border);
                return Some(*neighbor_id);
            }
        }
        None
    }
}

fn flip(image: Vec<Vec<char>>) -> Vec<Vec<char>> {
    image.into_iter().rev().collect()
}

fn rotate(image: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = image.len();
    let mut new_image = vec![vec!['.'; n]; n];

    for i in 0..n {
        for j in 0..n {
            new_image[i][j] = image[j][n - 1 - i];
        }
    }
    new_image
}

fn is_monster_here(image: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    MONSTER_OFFSETS.iter().all(|offset| {
        let row = row + offset.0;
        let col = col + offset.1;
        row < image.len() && col < image[row].len() && image[row][col] == '#'
    })
}

fn count_monsters(mut image: Vec<Vec<char>>) -> usize {
    let mut monsters = 0;
    for _ in 0..4 {
        for row in 0..image.len() {
            for col in 0..image[0].len() {
                if is_monster_here(&image, row, col) {
                    monsters += 1;
                }
            }
        }
        if monsters > 0 {
            return monsters;
        }
        image = rotate(image);
    }
    image = flip(image);
    for _ in 0..4 {
        for row in 0..image.len() {
            for col in 0..image[0].len() {
                if is_monster_here(&image, row, col) {
                    monsters += 1;
                }
            }
        }
        if monsters > 0 {
            return monsters;
        }
        image = rotate(image);
    }
    unreachable!()
}

pub fn main() {
    println!("Day 20");
    let tiles = read_block("../inputs/day20.txt");
    let mut solution = Solution::new(tiles);
    println!("\tPart1: {}", solution.solve_part_1());
    println!("\tPart2: {}", solution.solve_part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_input = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;
        let mut solution = Solution::new(test_input.split("\n\n").map(|s| s.to_string()).collect());
        assert_eq!(solution.solve_part_1(), 20899048083289);
        assert_eq!(solution.solve_part_2(), 273);
    }
}
