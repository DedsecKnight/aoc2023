use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day15;

enum Operation<'a> {
    Remove(usize, &'a str),
    Add(usize, &'a str, u32),
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
            .filter(|elem| !elem.is_empty())
            .collect::<Vec<_>>();
        if parsed_raw_op.len() == 2 {
            Operation::Add(
                Self::calculate_hash(parsed_raw_op[0]) as usize,
                parsed_raw_op[0],
                parsed_raw_op[1].parse::<u32>().unwrap(),
            )
        } else {
            assert!(parsed_raw_op.len() == 1);
            Operation::Remove(
                Self::calculate_hash(parsed_raw_op[0]) as usize,
                parsed_raw_op[0],
            )
        }
    }
}

impl Solution<u32, u32> for Day15 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        lines
            .into_iter()
            .next()
            .unwrap()
            .split(',')
            .map(Self::calculate_hash)
            .sum()
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut boxes: Vec<Vec<(&str, u32)>> = vec![Vec::new(); 256];
        let operations = lines
            .into_iter()
            .next()
            .unwrap()
            .split(',')
            .map(Self::parse_operation)
            .collect::<Vec<_>>();
        for operation in operations.into_iter() {
            match operation {
                Operation::Add(box_index, key, value) => {
                    if let Some(target_index) = boxes[box_index]
                        .iter()
                        .enumerate()
                        .filter(|elem| elem.1 .0 == key)
                        .map(|elem| elem.0.to_owned())
                        .next()
                    {
                        boxes[box_index][target_index].1 = value;
                    } else {
                        boxes[box_index].push((key, value));
                    }
                }
                Operation::Remove(box_index, key) => {
                    if let Some(target_index) = boxes[box_index]
                        .iter()
                        .enumerate()
                        .filter(|elem| elem.1 .0 == key)
                        .map(|elem| elem.0.to_owned())
                        .next()
                    {
                        boxes[box_index].drain(target_index..target_index + 1);
                    }
                }
            }
        }
        boxes
            .into_iter()
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
            .sum()
    }
}
