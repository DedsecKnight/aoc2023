use crate::solution::Solution;
use macros::return_type;
use std::collections::HashMap;

#[return_type(p1 = u32, p2 = u64)]
pub struct Day18;

impl Day18 {
    fn parse_line(line: &str) -> (char, u32, String) {
        let splitted = line.split(' ').collect::<Vec<_>>();
        (splitted[0].chars().next().unwrap(), splitted[1].parse::<u32>().unwrap(), splitted[2].replace(['(', ')'], ""))
    }   
}

impl Solution<u32, u64> for Day18 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut points: Vec<(i32, i32)> = Vec::new();
        let mut point_color: HashMap<(i32, i32), String> = HashMap::new();
        let parsed_entries = lines.map(Self::parse_line);
        let mut current_point = (0i32, 0i32);        
        
        for entry in parsed_entries {
            for _iter in 0..entry.1 {
                current_point = match entry.0 {
                    'U' => (current_point.0, current_point.1 + 1),
                    'R' => (current_point.0 + 1, current_point.1),
                    'L' => (current_point.0 - 1, current_point.1),
                    'D' => (current_point.0, current_point.1 - 1),
                    _ => {
                        panic!("Invalid direction found: {}", entry.0)
                    }
                };
                points.push(current_point);
                assert!(!point_color.contains_key(&current_point));
                point_color.insert(current_point.clone(), entry.2.clone());
            }
        }
        let area = (points.iter().enumerate().fold(0i32, |acc, curr| {
            let p = if curr.0 == 0 {
                points.last().unwrap()
            } else {
                &points[curr.0-1]
            };
            acc + (p.0 - curr.1.0) * (p.1 + curr.1.1)
        }).abs() / 2) as u32;
        let internal = area + 1 - (((points.len() / 2)) as u32);
        points.len()  as u32 + internal
        
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
        let mut points: Vec<(i64, i64)> = Vec::new();
        let dir_list = vec!['R', 'D', 'L', 'U'];
        let mut current_point = (0i64, 0i64);        
        let mut num_total_points = 0i64;
        for line in lines {
            let parsed_inst = Self::parse_line(line);
            let mut hex_data = parsed_inst.2.chars().collect::<Vec<_>>();
            hex_data.drain(0..1);
            let dir = dir_list[hex_data.pop().unwrap().to_digit(10).unwrap() as usize];
            let delta = i64::from_str_radix(hex_data.into_iter().fold(String::new(), |mut acc, curr| { 
                acc.push(curr);
                acc
            }).as_str(), 16).unwrap();
            current_point = match dir {
                'U' => (current_point.0, current_point.1 + delta),
                'R' => (current_point.0 + delta, current_point.1),
                'L' => (current_point.0 - delta, current_point.1),
                'D' => (current_point.0, current_point.1 - delta),
                _ => {
                    panic!("Invalid direction found: {}", dir)
                }
            };
            points.push(current_point);
            num_total_points += delta;
        }
        (points.iter().enumerate().fold(0i64, |acc, curr| {
            let p = if curr.0 == 0 {
                points.last().unwrap()
            } else {
                &points[curr.0-1]
            };
            acc + (p.0 - curr.1.0) * (p.1 + curr.1.1)
        }).abs() / 2) as u64 + 1 + ((num_total_points / 2) as u64)
    }
}
