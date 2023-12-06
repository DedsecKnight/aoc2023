use std::iter;

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = i64)]
pub struct Day6;

struct QuadraticEquation {
    a: f64,
    b: f64,
    c: f64,
}

impl QuadraticEquation {
    fn find_clamped_roots(&self, lb: f64, rb: f64) -> (f64, f64) {
        let delta = self.b * self.b - 4.0 * self.a * self.c;
        assert!(delta >= 0.0);
        (
            f64::max((-self.b + f64::sqrt(delta)) / (2.0 * self.a), lb).ceil(),
            f64::min((-self.b - f64::sqrt(delta)) / (2.0 * self.a), rb).floor(),
        )
    }
    fn evaluate(&self, x: f64) -> f64 {
        self.a * x.powi(2) + self.b * x + self.c
    }
    fn new(param_a: f64, param_b: f64, param_c: f64) -> QuadraticEquation {
        QuadraticEquation {
            a: param_a,
            b: param_b,
            c: param_c,
        }
    }
}

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
}

impl Solution<u32, i64> for Day6 {
    fn part_one<'a>(_lines: impl Iterator<Item = &'a str>) -> u32 {
        let lines = _lines.collect::<Vec<_>>();
        iter::zip(Day6::parse_line(lines[0]), Day6::parse_line(lines[1])).fold(1, |acc, curr| {
            ((0..=curr.0).filter(|x| (curr.0 - x) * x > curr.1).count() as u32) * acc
        })
    }
    fn part_two<'a>(_lines: impl Iterator<Item = &'a str>) -> i64 {
        let lines = _lines.collect::<Vec<_>>();
        let time = Day6::join_line(Day6::parse_line(lines[0]));
        let distance = Day6::join_line(Day6::parse_line(lines[1]));
        let f = QuadraticEquation::new(-1 as f64, time as f64, -distance as f64);
        let (mut lb, mut rb) = f.find_clamped_roots(0.0, time as f64);
        if f.evaluate(lb) <= 0.0 {
            lb = lb + 1.0;
        }
        if f.evaluate(rb) <= 0.0 {
            rb = rb - 1.0;
        }
        (rb as i64) - (lb as i64) + 1
    }
}
