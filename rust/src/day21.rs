use std::collections::{HashMap, HashSet};

use crate::utilities::read_lines;

struct Solution {
    ingredients: HashMap<String, HashSet<String>>,
    allergens: HashMap<String, HashSet<String>>,
    ingredient_counts: HashMap<String, i32>,
    total_ingredients: i32,
}

impl Solution {
    fn new(foods: Vec<String>) -> Self {
        let mut ingredients: HashMap<String, HashSet<String>> = HashMap::new();
        let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();
        let mut ingredient_counts = HashMap::new();
        let mut total_ingredients = 0;
        for food in foods {
            let mut iter = food.split(" (");
            let food_ingredients: HashSet<String> = iter
                .next()
                .unwrap()
                .split(' ')
                .map(|ingredient| ingredient.to_string())
                .collect();
            let food_allergens: HashSet<String> = iter
                .next()
                .unwrap()
                .replace(")", "")
                .replace("contains ", "")
                .split(", ")
                .map(|allergen| allergen.to_string())
                .collect();

            for ingredient in &food_ingredients {
                match ingredients.get_mut(ingredient) {
                    Some(allergens) => {
                        *allergens = allergens
                            .union(&food_allergens)
                            .map(|s| s.clone())
                            .collect();
                    }
                    None => {
                        ingredients.insert(ingredient.clone(), food_allergens.clone());
                    }
                }
                *ingredient_counts.entry(ingredient.clone()).or_insert(0) += 1;
                total_ingredients += 1;
            }

            for allergen in &food_allergens {
                match allergens.get_mut(allergen) {
                    Some(ingredients) => {
                        *ingredients = ingredients
                            .intersection(&food_ingredients)
                            .map(|s| s.clone())
                            .collect();
                    }
                    None => {
                        allergens.insert(allergen.clone(), food_ingredients.clone());
                    }
                }
            }
        }
        Self {
            ingredients,
            allergens,
            ingredient_counts,
            total_ingredients,
        }
    }

    fn solve_part_1(&mut self) -> i32 {
        let mut seen = HashSet::new();
        let mut stack = vec![];
        loop {
            for (allergen, ingredients) in &self.allergens {
                if ingredients.len() == 1 && !seen.contains(allergen) {
                    seen.insert(allergen.clone());
                    stack.push(allergen.clone());
                }
            }
            if stack.is_empty() {
                break;
            }

            while let Some(allergen) = stack.pop() {
                let ingredient = self
                    .allergens
                    .get(&allergen)
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap()
                    .to_string();
                for other_allergen in self.ingredients.get(&ingredient).unwrap() {
                    if other_allergen != &allergen {
                        self.allergens
                            .get_mut(other_allergen)
                            .unwrap()
                            .remove(&ingredient);
                    }
                }
            }
        }
        let mut valid_ingredients_count = 0;
        for ingredients in self.allergens.values() {
            for ingredient in ingredients {
                valid_ingredients_count += self.ingredient_counts.get(ingredient).unwrap();
            }
        }
        self.total_ingredients - valid_ingredients_count
    }

    fn solve_part_2(&self) -> String {
        let mut pairs = vec![];
        for (allergen, ingredients) in &self.allergens {
            for ingredient in ingredients {
                pairs.push((allergen, ingredient))
            }
        }
        pairs.sort();
        pairs
            .into_iter()
            .map(|pair| pair.1.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .to_string()
    }
}

pub fn main() {
    println!("Day 21");
    let foods = read_lines("../inputs/day21.txt");
    let mut solution = Solution::new(foods);
    println!("\tPart1: {}", solution.solve_part_1());
    println!("\tPart2: {}", solution.solve_part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_input = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;
        let mut solution = Solution::new(test_input.lines().map(str::to_string).collect());
        assert_eq!(solution.solve_part_1(), 5);
        assert_eq!(solution.solve_part_2(), "mxmxvkd,sqjhc,fvjkl".to_string());
    }
}
