use macros::return_type;
use std::iter;

use crate::solution::Solution;

#[return_type(p1 = u64, p2 = u64)]
pub struct Day12;

impl Day12 {
    fn parse_line(line: &str, num_copies: usize) -> (String, Vec<u32>) {
        let splitted_line = line.split(' ').collect::<Vec<_>>();
        let mut config_list = splitted_line[1]
            .split(',')
            .flat_map(|x| x.parse::<u32>())
            .collect::<Vec<_>>();
        let n = config_list.len();
        for _ in 1..num_copies {
            for i in 0..n {
                config_list.push(config_list[i]);
            }
        }
        (
            iter::repeat(splitted_line[0])
                .take(num_copies)
                .collect::<Vec<_>>()
                .join("?"),
            config_list,
        )
    }
    fn validate_configuration(config: &Vec<char>, requirement: &Vec<u32>) -> bool {
        let mut broken_sequence: Vec<u32> = Vec::new();
        let mut ptr = 0;
        for i in 0..config.len() {
            if config[i] != config[ptr] {
                if config[ptr] == '#' {
                    broken_sequence.push((i - ptr) as u32);
                }
                ptr = i;
            }
        }
        if config[ptr] == '#' {
            broken_sequence.push((config.len() - ptr) as u32);
        }
        if broken_sequence.len() != requirement.len() {
            return false;
        }
        broken_sequence
            .iter()
            .enumerate()
            .all(|elem| requirement[elem.0] == *elem.1)
    }
    fn count_valid_arrangement_bruteforce(parsed_line: (String, Vec<u32>)) -> u64 {
        let mut config_chars = parsed_line.0.chars().collect::<Vec<_>>();
        let question_mark_pos = config_chars
            .iter()
            .enumerate()
            .filter(|x| x.1 == &'?')
            .map(|x| x.0)
            .collect::<Vec<_>>();
        (0..(1 << question_mark_pos.len()))
            .filter(|mask| {
                for i in 0..question_mark_pos.len() {
                    if (mask & (1 << i)) != 0 {
                        config_chars[question_mark_pos[i]] = '#';
                    } else {
                        config_chars[question_mark_pos[i]] = '.';
                    }
                }
                Self::validate_configuration(&config_chars, &parsed_line.1)
            })
            .count() as u64
    }
    fn count_valid_arrangement_optimized(parsed_line: (String, Vec<u32>)) -> u64 {
        let config_str = parsed_line.0.chars().collect::<Vec<_>>();
        let mut dp: Vec<Vec<u64>> = vec![vec![0; parsed_line.1.len() + 1]; config_str.len() + 1];
        dp[0][0] = 1;
        for i in 1..=config_str.len() {
            if config_str[i - 1] != '#' {
                dp[i][0] = dp[i - 1][0];
            }
        }
        let replaceable = |i: usize, j: usize| -> bool {
            if i < parsed_line.1[j - 1] as usize {
                return false;
            }
            for z in (i - parsed_line.1[j - 1] as usize)..i {
                if config_str[z] == '.' {
                    return false;
                }
            }
            if (i as u32) - parsed_line.1[j - 1] >= 1
                && config_str[i - (parsed_line.1[j - 1] as usize) - 1] == '#'
            {
                return false;
            }
            true
        };
        for i in 1..=config_str.len() {
            for j in 1..=parsed_line.1.len() {
                if replaceable(i, j) {
                    if i - (parsed_line.1[j - 1] as usize) >= 1 {
                        dp[i][j] += dp[i - (parsed_line.1[j - 1] as usize) - 1][j - 1];
                    } else {
                        dp[i][j] += dp[i - (parsed_line.1[j - 1] as usize)][j - 1];
                    }
                    if config_str[i - 1] == '?' {
                        dp[i][j] += dp[i - 1][j];
                    }
                } else if config_str[i - 1] != '#' {
                    dp[i][j] += dp[i - 1][j];
                }
            }
        }
        *dp.last().unwrap().last().unwrap()
    }
}

impl Solution<u64, u64> for Day12 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
        lines
            .map(|line| Self::count_valid_arrangement_bruteforce(Self::parse_line(line, 1)))
            .sum()
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
        lines
            .map(|line| Self::count_valid_arrangement_optimized(Self::parse_line(line, 5)))
            .sum()
    }
}
