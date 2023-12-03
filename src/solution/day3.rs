use std::collections::HashSet;

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day3;

impl Day3 {
    fn check_range(
        row: usize,
        left: usize,
        right: usize,
        clist: &Vec<char>,
        lines_list: &Vec<Vec<char>>,
    ) -> bool {
        let mut valid_number = false;
        let top = if row >= 1 { row - 1 } else { row };
        let bottom = if row + 1 < lines_list.len() {
            row + 1
        } else {
            row
        };
        for k in top..=bottom {
            if right < clist.len() {
                valid_number = valid_number || (lines_list[k][right] != '.');
            }
            if left >= 1 {
                valid_number = valid_number || (lines_list[k][left - 1] != '.');
            }
        }
        for k in left..right {
            if row >= 1 {
                valid_number = valid_number || lines_list[row - 1][k] != '.';
            }
            if row + 1 < lines_list.len() {
                valid_number = valid_number || lines_list[row + 1][k] != '.';
            }
        }
        valid_number
    }
    fn calculate_gear_ratio(
        lines_list: &Vec<Vec<char>>,
        region_mark: &Vec<Vec<Option<usize>>>,
        valid_numbers: &Vec<u32>,
        row: i32,
        col: i32,
    ) -> u32 {
        let mut seen: HashSet<u32> = HashSet::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if row + dx < 0
                    || row + dx >= lines_list.len().try_into().unwrap()
                    || col + dy < 0
                    || col + dy >= lines_list[row as usize].len().try_into().unwrap()
                {
                    continue;
                }
                if let Some(index) = region_mark[(row + dx) as usize][(col + dy) as usize] {
                    seen.insert(valid_numbers[index]);
                }
            }
        }
        if seen.len() != 2 {
            0
        } else {
            seen.iter().fold(1, |acc, curr| acc * curr)
        }
    }
}

impl Solution<u32, u32> for Day3 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let lines_list = lines
            .map(|line| line.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut ret = 0;
        for (j, clist) in lines_list.iter().enumerate() {
            let mut ptr: usize = 0;
            let mut curr_val = 0;
            for (i, c) in clist.iter().enumerate() {
                if c.is_digit(10) {
                    curr_val = curr_val * 10 + c.to_digit(10).unwrap();
                    if !clist[ptr].is_digit(10) {
                        ptr = i;
                    }
                } else {
                    if clist[ptr].is_digit(10) {
                        if Day3::check_range(j, ptr, i, clist, &lines_list) {
                            ret += curr_val;
                        }
                    }
                    curr_val = 0;
                    ptr = i;
                }
            }
            if curr_val > 0 && Day3::check_range(j, ptr, clist.len(), clist, &lines_list) {
                ret += curr_val;
            }
        }
        ret
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let lines_list = lines
            .map(|line| line.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut region_mark: Vec<Vec<Option<usize>>> =
            vec![vec![None; lines_list[0].len()]; lines_list.len()];
        let mut valid_numbers: Vec<u32> = Vec::new();
        let mut ret = 0;
        let mut mark_range = |j: usize, ptr: usize, i: usize, region_index: usize| {
            for k in ptr..i {
                region_mark[j][k] = Some(region_index);
            }
        };
        for (j, clist) in lines_list.iter().enumerate() {
            let mut ptr: usize = 0;
            let mut curr_val = 0;
            for (i, c) in clist.iter().enumerate() {
                if c.is_digit(10) {
                    curr_val = curr_val * 10 + c.to_digit(10).unwrap();
                    if !clist[ptr].is_digit(10) {
                        ptr = i;
                    }
                } else {
                    if clist[ptr].is_digit(10) {
                        if Day3::check_range(j, ptr, i, clist, &lines_list) {
                            mark_range(j, ptr, i, valid_numbers.len());
                            valid_numbers.push(curr_val);
                        }
                    }
                    curr_val = 0;
                    ptr = i;
                }
            }
            if curr_val > 0 && Day3::check_range(j, ptr, clist.len(), clist, &lines_list) {
                mark_range(j, ptr, clist.len(), valid_numbers.len());
                valid_numbers.push(curr_val);
            }
        }
        for (j, clist) in lines_list.iter().enumerate() {
            for (i, c) in clist.iter().enumerate() {
                if c == &'*' {
                    ret += Day3::calculate_gear_ratio(
                        &lines_list,
                        &region_mark,
                        &valid_numbers,
                        j.try_into().unwrap(),
                        i.try_into().unwrap(),
                    );

                }
            }
        }
        ret
    }
}
