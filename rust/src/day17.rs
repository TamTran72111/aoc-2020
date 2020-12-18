use crate::utilities::read_lines;
use std::collections::HashSet;

fn detect_new_cube_state(
    state: &HashSet<(i32, i32, i32, i32)>,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
) -> bool {
    let range = [-1, 0, 1];
    let mut active_neighbours = 0;
    for &i in &range {
        for &j in &range {
            for &k in &range {
                for &l in &range {
                    if i == 0 && j == 0 && k == 0 && l == 0 {
                        // Not count itself
                        continue;
                    }
                    if state.contains(&(x + i, y + j, z + k, w + l)) {
                        active_neighbours += 1;
                    }
                }
            }
        }
    }
    active_neighbours == 3 || (active_neighbours == 2 && state.contains(&(x, y, z, w)))
}

fn conway_cubes(initial: &Vec<String>, is_3d: bool) -> usize {
    let mut state = HashSet::new();
    let mut new_state = HashSet::new();
    for (y, row) in initial.iter().enumerate() {
        for (x, value) in row.chars().enumerate() {
            if value == '#' {
                state.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    let (mut min_x, mut min_y, mut min_z, mut min_w) = (-1, -1, -1, -1);
    let (mut max_x, mut max_y, mut max_z, mut max_w) =
        (initial[0].len() as i32, initial.len() as i32, 1, 1);
    if is_3d {
        min_w = 0;
        max_w = 0;
    }

    // Perform 6 cycles
    for _ in 0..6 {
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                for z in min_z..max_z + 1 {
                    for w in min_w..max_w + 1 {
                        if detect_new_cube_state(&state, x, y, z, w) {
                            new_state.insert((x, y, z, w));
                        } else {
                            new_state.remove(&(x, y, z, w));
                        }
                    }
                }
            }
        }
        // After each cycle, the range will expand atmost 1 unit to each direction.
        min_x -= 1;
        max_x += 1;
        min_y -= 1;
        max_y += 1;
        min_z -= 1;
        max_z += 1;
        // If it is 3d, so just update the other 3 dimensions.
        min_w -= if is_3d { 0 } else { 1 };
        max_w += if is_3d { 0 } else { 1 };

        std::mem::swap(&mut state, &mut new_state);
    }
    state.len()
}

pub fn main() {
    println!("Day 17");
    let initial = read_lines("../inputs/day17.txt");

    println!("\tPart 1: {}", conway_cubes(&initial, true));
    println!("\tPart 2: {}", conway_cubes(&initial, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3d() {
        let initial = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];

        assert_eq!(conway_cubes(&initial, true), 112);
    }

    #[test]
    fn test_4d() {
        let initial = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];

        assert_eq!(conway_cubes(&initial, false), 848);
    }
}
