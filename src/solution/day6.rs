use std::iter;

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = i64)]
pub struct Day6;

impl Day6 {
    fn parse_line(line: &str) -> Vec<u32> {
        line.split(' ').flat_map(|x| x.parse::<u32>()).collect()
    }
    fn join_line(num_list: Vec<u32>) -> i64 {
        num_list
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse::<i64>()
            .unwrap()
    }
    // find quadratic root of ax^2 + bx + c = 0
    fn find_quadratic_root(a: f64, b: f64, c: f64) -> (f64, f64) {
        let delta = b * b - 4.0 * a * c;
        assert!(delta >= 0.0);
        (
            f64::max((-b + f64::sqrt(delta)) / (2.0 * a), 0.0).ceil(),
            f64::min((-b - f64::sqrt(delta)) / (2.0 * a), b).floor(),
        )
    }
    fn evaluate_quadratic_equation(a: f64, b: f64, c: f64, x: f64) -> f64 {
        a * x * x + b * x + c
    }
}

impl Solution<u32, i64> for Day6 {
    fn part_one<'a>(_lines: impl Iterator<Item = &'a str>) -> u32 {
        let lines = _lines.collect::<Vec<_>>();
        let races =
            iter::zip(Day6::parse_line(lines[0]), Day6::parse_line(lines[1])).collect::<Vec<_>>();
        races.into_iter().fold(1, |acc, curr| {
            ((0..=curr.0).filter(|x| (curr.0 - x) * x > curr.1).count() as u32) * acc
        })
    }
    fn part_two<'a>(_lines: impl Iterator<Item = &'a str>) -> i64 {
        let lines = _lines.collect::<Vec<_>>();
        let time = Day6::join_line(Day6::parse_line(lines[0]));
        let distance = Day6::join_line(Day6::parse_line(lines[1]));
        let (mut lb, mut rb) = Day6::find_quadratic_root(-1 as f64, time as f64, -distance as f64);
        if Day6::evaluate_quadratic_equation(-1 as f64, time as f64, -distance as f64, lb)
            <= 0 as f64
        {
            lb = lb + 1.0;
        }
        if Day6::evaluate_quadratic_equation(-1 as f64, time as f64, -distance as f64, rb)
            <= 0 as f64
        {
            rb = rb - 1.0;
        }
        (rb as i64) - (lb as i64) + 1
    }
}
