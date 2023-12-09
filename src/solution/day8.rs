use std::collections::{HashMap, HashSet};

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u64)]
pub struct Day8;

impl Day8 {
    fn parse_edge(raw_edge_data: &str) -> (String, Vec<String>) {
        let intermediary_1 = raw_edge_data.split('=').collect::<Vec<_>>();
        let intermediary_2 = intermediary_1[1]
            .trim()
            .split(',')
            .map(|x| x.replace("(", "").replace(")", "").replace(" ", ""))
            .collect::<Vec<_>>();
        (
            String::from(intermediary_1[0].replace(" ", "")),
            vec![intermediary_2[0].to_owned(), intermediary_2[1].to_owned()],
        )
    }
    fn find_dist(
        mut curr_node: String,
        instruction: &Vec<char>,
        destination_set: &HashSet<String>,
        node_mapping: &HashMap<String, Vec<String>>,
    ) -> u32 {
        let mut index = 0;
        let mut counter = 0;
        while !destination_set.contains(&curr_node) {
            if instruction[index] == 'L' {
                curr_node = node_mapping
                    .get(&curr_node)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .clone();
            } else {
                curr_node = node_mapping
                    .get(&curr_node)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .clone();
            }
            counter += 1;
            index = (index + 1) % instruction.len();
        }
        counter
    }
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
    fn lcm(a: u64, b: u64) -> u64 {
        a / Self::gcd(a, b) * b
    }
}

impl Solution<u32, u64> for Day8 {
    fn part_one<'a>(lines_it: impl Iterator<Item = &'a str>) -> u32 {
        let lines = lines_it.collect::<Vec<_>>();
        let instruction = lines[0].chars().collect::<Vec<_>>();
        let mut node_mapping: HashMap<String, Vec<String>> = HashMap::new();
        for i in 2..lines.len() {
            let (from, to) = Self::parse_edge(lines[i]);
            node_mapping.insert(from, to);
        }
        let curr_node = String::from("AAA");
        let mut destination_set = HashSet::new();
        destination_set.insert(String::from("ZZZ"));
        Self::find_dist(curr_node, &instruction, &destination_set, &node_mapping)
    }
    fn part_two<'a>(lines_it: impl Iterator<Item = &'a str>) -> u64 {
        let lines = lines_it.collect::<Vec<_>>();
        let instruction = lines[0].chars().collect::<Vec<_>>();
        let mut node_mapping: HashMap<String, Vec<String>> = HashMap::new();
        let mut destination_set: HashSet<String> = HashSet::new();
        let mut source_list: Vec<String> = Vec::new();
        for i in 2..lines.len() {
            let (from, to) = Self::parse_edge(lines[i]);
            if from.ends_with('Z') {
                destination_set.insert(from.clone());
            } else if from.ends_with('A') {
                source_list.push(from.clone());
            }
            node_mapping.insert(from, to);
        }
        source_list.into_iter().fold(1u64, |acc, curr| {
            Self::lcm(
                acc,
                Self::find_dist(curr, &instruction, &destination_set, &node_mapping) as u64,
            )
        })
    }
}
