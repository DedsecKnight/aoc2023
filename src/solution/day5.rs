use std::collections::VecDeque;

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = i64, p2 = i64)]
pub struct Day5;

enum QueueElement {
    Value((i64, i64)),
    Sentinel,
}

impl Day5 {
    fn parse_seed_data(seed_data: &str) -> Vec<i64> {
        seed_data
            .split(' ')
            .flat_map(|x| x.parse::<i64>())
            .collect()
    }
    fn parse_seed_data_p2(raw_seed_data: &Vec<i64>) -> Vec<(i64, i64)> {
        let mut ret = Vec::new();
        let mut index = 0;
        while index < raw_seed_data.len() {
            ret.push((
                raw_seed_data[index],
                raw_seed_data[index] + raw_seed_data[index + 1] - 1,
            ));
            index += 2;
        }
        ret
    }
    fn parse_map<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<(i64, i64, i64)> {
        let mut ret: Vec<(i64, i64, i64)> = Vec::new();
        loop {
            let it = lines.next();
            if it.is_none() {
                break;
            }
            let num_data = it
                .unwrap()
                .split(' ')
                .flat_map(|x| x.parse::<i64>())
                .collect::<Vec<_>>();
            if num_data.is_empty() {
                break;
            }
            assert!(num_data.len() == 3);
            ret.push((num_data[0], num_data[1], num_data[2]));
        }
        lines.next();
        ret
    }
}

impl Solution<i64, i64> for Day5 {
    fn part_one<'a>(_lines: impl Iterator<Item = &'a str>) -> i64 {
        let lines_list = _lines.collect::<Vec<_>>();
        let mut lines = lines_list.into_iter();
        let seeds = Day5::parse_seed_data(lines.next().unwrap());
        lines.nth(1).unwrap();
        let mappings = (0..7)
            .map(|_| Day5::parse_map(&mut lines))
            .collect::<Vec<_>>();
        seeds.iter().fold(i64::MAX, |acc, seed_value| {
            std::cmp::min(
                acc,
                mappings.iter().fold(seed_value.clone(), |inner_acc, curr| {
                    curr.iter()
                        .filter(|x| inner_acc >= x.1 && inner_acc < x.1 + x.2)
                        .fold(inner_acc, |_, z| z.0 + inner_acc - z.1)
                }),
            )
        })
    }
    fn part_two<'a>(_lines: impl Iterator<Item = &'a str>) -> i64 {
        let lines_list = _lines.collect::<Vec<_>>();
        let mut lines = lines_list.into_iter();
        let raw_seeds_data = Day5::parse_seed_data(lines.next().unwrap());
        let mut seeds_data: Vec<(i64, i64)> = Day5::parse_seed_data_p2(&raw_seeds_data);
        lines.nth(1).unwrap();
        let mappings: Vec<Vec<(i64, i64, i64)>> =
            (0..7).map(|_| Day5::parse_map(&mut lines)).collect();
        let mut remaining_ranges: VecDeque<QueueElement> = VecDeque::new();
        for mapping in mappings.iter() {
            let mut new_seeds_data = Vec::new();
            for seed_range in seeds_data.into_iter() {
                remaining_ranges.push_back(QueueElement::Value(seed_range));
                for mapping_range in mapping.iter() {
                    remaining_ranges.push_back(QueueElement::Sentinel);
                    while let Some(QueueElement::Value(curr_seed_range)) =
                        remaining_ranges.pop_front()
                    {
                        if curr_seed_range.0 >= mapping_range.1 + mapping_range.2
                            || curr_seed_range.1 < mapping_range.1
                        {
                            remaining_ranges.push_back(QueueElement::Value(curr_seed_range));
                            continue;
                        }
                        let lb = std::cmp::max(curr_seed_range.0, mapping_range.1);
                        let rb =
                            std::cmp::min(curr_seed_range.1, mapping_range.1 + mapping_range.2 - 1);
                        if lb > curr_seed_range.0 {
                            remaining_ranges
                                .push_back(QueueElement::Value((curr_seed_range.0, lb - 1)));
                        }
                        new_seeds_data.push((
                            lb - mapping_range.1 + mapping_range.0,
                            rb - mapping_range.1 + mapping_range.0,
                        ));
                        if rb < curr_seed_range.1 {
                            remaining_ranges
                                .push_back(QueueElement::Value((rb + 1, curr_seed_range.1)));
                        }
                    }
                }
                while let Some(QueueElement::Value(elem)) = remaining_ranges.pop_front() {
                    new_seeds_data.push(elem);
                }
            }
            seeds_data = Vec::from_iter(new_seeds_data.into_iter());
        }
        seeds_data
            .into_iter()
            .fold(i64::MAX, |acc, curr| std::cmp::min(acc, curr.0))
    }
}
