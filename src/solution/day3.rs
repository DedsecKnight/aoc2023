use std::collections::HashSet;

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day3;

impl Solution<u32, u32> for Day3 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let lines_list = lines.map(|line| line.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut ret = 0;
        let check_range = |j: usize, clist: &Vec<char>, ptr: usize, i: usize| -> bool {
            let mut valid_number = false;
            let lb = if j >= 1 { j-1 } else { j };
            let rb = if j+1 < lines_list.len() { j+1 } else { j };
            if ptr >= 1 {
                for k in lb..=rb {
                    valid_number = valid_number || (lines_list[k][ptr-1] != '.');
                }
            }
            if i < clist.len() {
                for k in lb..=rb {
                    valid_number = valid_number || (lines_list[k][i] != '.');
                }
            }
            for k in ptr..i {
                if j >= 1 {
                    valid_number = valid_number || lines_list[j-1][k] != '.';
                } 
                if j+1 < lines_list.len() {
                    valid_number = valid_number || lines_list[j+1][k] != '.';
                }
            }
            valid_number
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
                        if check_range(j, clist, ptr, i) {
                            ret += curr_val;
                        }
                    }
                    curr_val = 0;
                    ptr = i;
                }
            }
            if curr_val > 0 && check_range(j, clist, ptr, clist.len()) {
                ret += curr_val;
            }
        }
        ret
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let lines_list = lines.map(|line| line.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut region_mark: Vec<Vec<Option<usize>>> = vec![vec![None; lines_list[0].len()]; lines_list.len()];
        let mut valid_numbers: Vec<u32> = Vec::new();
        let mut ret = 0;
        let check_range = |j: usize, clist: &Vec<char>, ptr: usize, i: usize| -> bool {
            let mut valid_number = false;
            let lb = if j >= 1 { j-1 } else { j };
            let rb = if j+1 < lines_list.len() { j+1 } else { j };
            if ptr >= 1 {
                for k in lb..=rb {
                    valid_number = valid_number || (lines_list[k][ptr-1] != '.');
                }
            }
            if i < clist.len() {
                for k in lb..=rb {
                    valid_number = valid_number || (lines_list[k][i] != '.');
                }
            }
            for k in ptr..i {
                if j >= 1 {
                    valid_number = valid_number || lines_list[j-1][k] != '.';
                } 
                if j+1 < lines_list.len() {
                    valid_number = valid_number || lines_list[j+1][k] != '.';
                }
            }
            valid_number
        };
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
                        if check_range(j, clist, ptr, i) {
                            mark_range(j, ptr, i, valid_numbers.len());
                            valid_numbers.push(curr_val);
                        }
                    }
                    curr_val = 0;
                    ptr = i;
                }
            }
            if curr_val > 0 && check_range(j, clist, ptr, clist.len()) {
                mark_range(j, ptr, clist.len(), valid_numbers.len());
                valid_numbers.push(curr_val);
            }
        }
        let evaluate = |j: usize, i: usize| -> u32 {
            let mut seen: HashSet<u32> = HashSet::new();
            if j >= 1 {
                if i >= 1 {
                    if let Some(index) = region_mark[j-1][i-1] {
                        seen.insert(valid_numbers[index]);
                    } 
                }
                if i+1 < lines_list[j].len() {
                    if let Some(index) = region_mark[j-1][i+1] {
                        seen.insert(valid_numbers[index]);
                    }
                }
                if let Some(index) = region_mark[j-1][i] {
                    seen.insert(valid_numbers[index]);
                }
            } 
            if j+1 < lines_list.len() {
                if i >= 1 {
                    if let Some(index) = region_mark[j+1][i-1] {
                        seen.insert(valid_numbers[index]);
                    }
                } 
                if i+1 < lines_list[j].len() {
                    if let Some(index) = region_mark[j+1][i+1] {
                        seen.insert(valid_numbers[index]);
                    }
                }
                if let Some(index) = region_mark[j+1][i] {
                    seen.insert(valid_numbers[index]);
                }
            }
            if i >= 1 {
                if let Some(index) = region_mark[j][i-1] {
                    seen.insert(valid_numbers[index]);
                }
            } 
            if i+1 < lines_list[j].len() {
                if let Some(index) = region_mark[j][i+1] {
                    seen.insert(valid_numbers[index]);
                }
            }
            if seen.len() != 2 {
                0
            } else {
                seen.iter().fold(1, |acc, curr| acc * curr)
            }
        };
        for (j, clist) in lines_list.iter().enumerate() {
            for (i, c) in clist.iter().enumerate() {
                match c {
                    '*' => {
                        ret += evaluate(j, i);
                    },
                    &_ => {}
                }
            }
        }
        ret
    }
}
