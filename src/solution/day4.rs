use std::collections::HashSet;

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day4;

impl Day4 {
    fn parse_card_data(raw_card_data: &str) -> u32 {
        let colon_index = raw_card_data.find(':').unwrap();
        let pipe_index = raw_card_data.find('|').unwrap();
        let winning_numbers: HashSet<u32> = HashSet::from_iter(
            raw_card_data[colon_index + 2..pipe_index - 1]
                .trim()
                .split(' ')
                .flat_map(|x| x.parse::<u32>()),
        );
        raw_card_data[pipe_index + 2..]
            .trim()
            .split(' ')
            .flat_map(|x| x.parse::<u32>())
            .filter(|x| winning_numbers.contains(x))
            .count() as u32
    }
}

impl Solution<u32, u32> for Day4 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        lines
            .map(|line| Day4::parse_card_data(line))
            .fold(0u32, |acc, curr| {
                if curr == 0 {
                    acc
                } else {
                    acc + (2u32).pow(curr - 1)
                }
            })
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut multiplier: Vec<u32> = Vec::new();
        for (i, line) in lines.enumerate() {
            while multiplier.len() <= i {
                multiplier.push(0);
            }
            multiplier[i] += 1;
            let num_winning = Day4::parse_card_data(line) as usize;
            for j in i + 1..=i + num_winning {
                while multiplier.len() <= j {
                    multiplier.push(0);
                }
                multiplier[j] += multiplier[i];
            }
        }
        multiplier.into_iter().sum()
    }
}
