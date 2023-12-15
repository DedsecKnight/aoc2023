use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day15;

enum Operation {
    Remove(usize, String),
    Add(usize, String, u32),
}

impl Day15 {
    fn calculate_hash(s: &str) -> u32 {
        s.chars().fold(0u32, |acc, curr| {
            (acc + (curr.to_ascii_lowercase() as u32)) * 17 % 256
        })
    }
    fn parse_operation(s: &str) -> Operation {
        let parsed_raw_op = s
            .trim()
            .split(['-', '='])
            .filter(|elem| *elem != "")
            .collect::<Vec<_>>();
        if parsed_raw_op.len() == 2 {
            Operation::Add(
                Self::calculate_hash(parsed_raw_op[0]) as usize,
                String::from(parsed_raw_op[0]),
                parsed_raw_op[1].parse::<u32>().unwrap(),
            )
        } else {
            assert!(parsed_raw_op.len() == 1);
            Operation::Remove(
                Self::calculate_hash(parsed_raw_op[0]) as usize,
                String::from(parsed_raw_op[0]),
            )
        }
    }
}

impl Solution<u32, u32> for Day15 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        for line in lines {
            return line.split(",").map(|x| Self::calculate_hash(x)).sum();
        }
        return 0;
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        for line in lines {
            let mut boxes: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];
            let operations = line
                .split(",")
                .map(|s| Self::parse_operation(s))
                .collect::<Vec<_>>();
            for operation in operations.into_iter() {
                match operation {
                    Operation::Add(box_index, key, value) => {
                        let new_elem = (key, value);
                        if boxes[box_index].iter().any(|elem| elem.0 == new_elem.0) {
                            boxes[box_index] = boxes[box_index]
                                .clone()
                                .into_iter()
                                .map(|elem| {
                                    if elem.0 == new_elem.0 {
                                        new_elem.clone()
                                    } else {
                                        elem
                                    }
                                })
                                .collect::<Vec<_>>();
                        } else {
                            boxes[box_index].push(new_elem);
                        }
                    }
                    Operation::Remove(box_index, key) => {
                        if boxes[box_index].iter().any(|elem| elem.0 == key) {
                            boxes[box_index] = boxes[box_index]
                                .clone()
                                .into_iter()
                                .filter(|elem| elem.0 != key)
                                .collect::<Vec<_>>();
                        }
                    }
                }
            }
            return boxes
                .iter()
                .enumerate()
                .map(|(box_index, curr_box)| {
                    curr_box
                        .into_iter()
                        .enumerate()
                        .map(|(focal_index, focal_len)| {
                            ((box_index + 1) as u32) * ((focal_index + 1) as u32) * (focal_len.1)
                        })
                        .sum::<u32>()
                })
                .sum();
        }
        0
    }
}
