use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = i32, p2 = i32)]
pub struct Day9;

impl Day9 {
    fn parse_line(line: &str) -> Vec<i32> {
        line.split(' ')
            .flat_map(|x| x.parse::<i32>())
            .collect::<Vec<_>>()
    }
    fn extrapolate_foward(nums: &Vec<i32>) -> i32 {
        if nums.iter().all(|x| x == &0) {
            return 0;
        }
        nums.last().unwrap()
            + Self::extrapolate_foward(
                &(1..nums.len())
                    .map(|index| nums[index] - nums[index - 1])
                    .collect::<Vec<_>>(),
            )
    }
    fn extrapolate_backward(nums: &Vec<i32>) -> i32 {
        if nums.iter().all(|x| x == &0) {
            return 0;
        }
        nums.first().unwrap()
            - Self::extrapolate_backward(
                &(1..nums.len())
                    .map(|index| nums[index] - nums[index - 1])
                    .collect::<Vec<_>>(),
            )
    }
}

impl Solution<i32, i32> for Day9 {
    fn part_one<'a>(lines_it: impl Iterator<Item = &'a str>) -> i32 {
        lines_it
            .map(|line| Self::extrapolate_foward(&Self::parse_line(line)))
            .sum()
    }
    fn part_two<'a>(lines_it: impl Iterator<Item = &'a str>) -> i32 {
        lines_it
            .map(|line| Self::extrapolate_backward(&Self::parse_line(line)))
            .sum()
    }
}
