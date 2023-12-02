use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day2;

#[derive(Debug)]
struct MatchData {
    red: u32,
    green: u32,
    blue: u32,
}

impl Day2 {
    fn parse_match_data(match_data: &str) -> MatchData {
        let mut ret = MatchData {
            red: 0,
            green: 0,
            blue: 0,
        };
        for color in match_data.split(',').map(|c| c.trim()) {
            let data: Vec<&str> = color.split(' ').collect();
            let num_balls = data[0].parse::<u32>().unwrap();
            match data[1] {
                "red" => ret.red = num_balls,
                "blue" => ret.blue = num_balls,
                "green" => ret.green = num_balls,
                &_ => {
                    panic!("Invalid color found: {}", data[1]);
                }
            }
        }
        ret
    }
    fn parse_game_data(game_data: &str) -> Vec<MatchData> {
        let colon_index = game_data.find(':').unwrap();
        game_data[colon_index + 1..]
            .split(';')
            .map(|raw_match_data| Day2::parse_match_data(raw_match_data.trim()))
            .collect()
    }
}

impl Solution<u32, u32> for Day2 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut ret = 0;
        for (i, line) in lines.enumerate() {
            let matches_data = Day2::parse_game_data(line);
            let valid_match = matches_data.iter().all(|match_data| {
                match_data.red <= 12 && match_data.blue <= 14 && match_data.green <= 13
            });
            if valid_match {
                ret += (i as u32) + 1;
            }
        }
        ret
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut ret = 0u32;
        for line in lines {
            let matches_data = Day2::parse_game_data(line);
            let max_red = matches_data
                .iter()
                .fold(0u32, |acc, curr| std::cmp::max(acc, curr.red));
            let max_green = matches_data
                .iter()
                .fold(0u32, |acc, curr| std::cmp::max(acc, curr.green));
            let max_blue = matches_data
                .iter()
                .fold(0u32, |acc, curr| std::cmp::max(acc, curr.blue));
            ret += max_red * max_green * max_blue;
        }
        ret
    }
}
