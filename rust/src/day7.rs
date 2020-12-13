use std::collections::{HashMap, HashSet, VecDeque};

use crate::utilities::read_lines;

struct Graph {
    parent_bags: HashMap<String, Vec<String>>,
    children_bags: HashMap<String, Vec<(usize, String)>>,
}

impl Graph {
    fn new(regulations: Vec<String>) -> Graph {
        let mut parent_bags: HashMap<String, Vec<String>> = HashMap::new();
        let mut children_bags: HashMap<String, Vec<(usize, String)>> = HashMap::new();

        let mut read_regulation = |regulation: String| {
            let mut parts = regulation.split(" contain ");
            let outer_bag_ = parts.next().unwrap();
            let outer_color_end = outer_bag_.rfind(' ').unwrap();
            let outer_bag = &outer_bag_[..outer_color_end];
            let inner_bags = parts.next().unwrap().split(", ");
            for bag in inner_bags {
                if bag.contains("no other") {
                    continue;
                }
                let color_start = bag.find(' ').unwrap() + 1;
                let color_end = bag.rfind(' ').unwrap();
                let color = &bag[color_start..color_end];

                let parent = parent_bags.entry(color.to_string()).or_default();
                parent.push(outer_bag.to_string());

                let num_of_bags: usize = bag[..color_start - 1].parse().unwrap();
                let children = children_bags.entry(outer_bag.to_string()).or_default();

                children.push((num_of_bags, color.to_string()));
            }
        };
        for regulation in regulations {
            read_regulation(regulation);
        }
        Graph {
            parent_bags,
            children_bags,
        }
    }

    fn count_outermost_bag(&self, color: &str) -> usize {
        let mut set = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(color);
        while let Some(color) = queue.pop_front() {
            if let Some(parents) = self.parent_bags.get(color) {
                for parent in parents {
                    if set.insert(parent) {
                        queue.push_back(parent);
                    }
                }
            }
        }
        set.len()
    }

    fn count_bag_inside(&self, color: &str) -> usize {
        let mut mem = HashMap::new();
        self.count_bag_helper(color, &mut mem)
    }

    fn count_bag_helper(&self, color: &str, mem: &mut HashMap<String, usize>) -> usize {
        if let Some(value) = mem.get(color) {
            return *value;
        }
        match self.children_bags.get(color) {
            None => 0,
            Some(children) => {
                let mut ans = 0;
                for child in children {
                    ans += child.0;
                    ans += child.0 * self.count_bag_helper(child.1.as_str(), mem);
                }
                mem.insert(color.to_string(), ans);
                ans
            }
        }
    }
}

fn part_1(graph: &Graph) {
    println!("\tPart 1: {}", graph.count_outermost_bag("shiny gold"));
}

fn part_2(graph: &Graph) {
    println!("\tPart 2: {}", graph.count_bag_inside("shiny gold"));
}

pub fn main() {
    println!("Day 7");
    let graph = Graph::new(read_lines("../inputs/day7.txt"));
    part_1(&graph);
    part_2(&graph);
}
