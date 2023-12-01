use crate::solution::Solution;
use std::collections::HashMap;

pub struct Day1;

impl Solution<u32, u32> for Day1 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut total = 0;
        for line in lines {
            let mut first_digit: Option<u32>= None;
            let mut last_digit: Option<u32> = None;
            for c in line.chars() {
                if c.is_digit(10) {
                    if let None = first_digit {
                        first_digit = c.to_digit(10);
                    }
                    last_digit = c.to_digit(10);
                }
            }
            if let None = first_digit { 
                continue; 
            }
            total += first_digit.unwrap() * 10 + last_digit.unwrap();
        }
        total
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut total = 0;
        let num_map: HashMap<&'static str, u32> = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8), 
            ("nine", 9)
        ]);
        for line in lines {
            let c_list: Vec<char> = line.chars().collect();
            let mut i = 0;
            let mut first_digit: Option<u32> = None;
            let mut last_digit: Option<u32> = None;
            while i < c_list.len() {
                if c_list[i].is_digit(10) {
                    first_digit = c_list[i].to_digit(10);
                    break;
                }
                let mut found = false;
                for (k, v) in &num_map {
                    if i+k.len() <= c_list.len() && String::from(*k) == String::from(&line[i..i+k.len()]) {
                        first_digit = Some(*v);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
                i += 1;
            }
            if let None = first_digit {
                println!("Invalid line: {}", line);
                continue;
            }
            i = c_list.len()-1;
            loop {
                if c_list[i].is_digit(10) {
                    last_digit = c_list[i].to_digit(10);
                    break;
                }
                let mut found = false;
                for (k, v) in &num_map {
                    if i+k.len() <= c_list.len() && String::from(*k) == String::from(&line[i..i+k.len()]) {
                        last_digit = Some(*v);
                        found = true;
                        break;
                    }
                }
                if found || i == 0 {
                    break;
                }
                i -= 1;
            }
            let curr_val = first_digit.unwrap() * 10 + last_digit.unwrap();
            total += curr_val;
        }
        total
    }

}
