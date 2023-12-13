use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day13;

struct PatternMap {
    grid: Vec<Vec<char>>,
}

impl PatternMap {
    fn new(buffer: &Vec<Vec<char>>) -> PatternMap {
        PatternMap {
            grid: buffer.clone(),
        }
    }
    fn match_column(&self, i: usize, j: usize) -> bool {
        (0..self.grid.len()).all(|k| self.grid[k][i] == self.grid[k][j])
    }
    fn match_row(&self, i: usize, j: usize) -> bool {
        (0..self.grid[0].len()).all(|k| self.grid[i][k] == self.grid[j][k])
    }
    fn summarize(
        &self,
        invalid_horizontal_reflection: u32,
        invalid_vertical_reflection: u32,
    ) -> (u32, u32) {
        let mut vertical_value = 0u32;
        let mut horizontal_value = 0u32;
        for i in 0..self.grid[0].len() - 1 {
            let mut l = i;
            let mut r = i + 1;
            loop {
                if !self.match_column(l, r) {
                    l += 1;
                    r -= 1;
                    break;
                }
                if l == 0 || r == self.grid[0].len() - 1 {
                    break;
                }
                l -= 1;
                r += 1;
            }
            if l <= r
                && (l == 0 || r == self.grid[0].len() - 1)
                && (i + 1) as u32 != invalid_vertical_reflection
            {
                vertical_value = (i + 1) as u32;
                break;
            }
        }
        for i in 0..self.grid.len() - 1 {
            let mut l = i;
            let mut r = i + 1;
            loop {
                if !self.match_row(l, r) {
                    l += 1;
                    r -= 1;
                    break;
                }
                if l == 0 || r == self.grid.len() - 1 {
                    break;
                }
                l -= 1;
                r += 1;
            }
            if l <= r
                && (l == 0 || r == self.grid.len() - 1)
                && (i + 1) as u32 != invalid_horizontal_reflection
            {
                horizontal_value = (i + 1) as u32;
            }
        }
        (vertical_value, horizontal_value)
    }
}

impl Day13 {
    fn read_pattern_list<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<PatternMap> {
        let mut current_buffer: Vec<Vec<char>> = Vec::new();
        let mut patterns: Vec<PatternMap> = Vec::new();
        for line in lines {
            if line == "" {
                patterns.push(PatternMap::new(&current_buffer));
                current_buffer.clear();
            } else {
                current_buffer.push(line.chars().collect::<Vec<_>>());
            }
        }
        if !current_buffer.is_empty() {
            patterns.push(PatternMap::new(&current_buffer));
        }
        patterns
    }
}

impl Solution<u32, u32> for Day13 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let patterns = Self::read_pattern_list(lines);
        patterns
            .into_iter()
            .map(|pattern| {
                let (vertical, horizontal) =
                    pattern.summarize(pattern.grid.len() as u32, pattern.grid[0].len() as u32);
                vertical + horizontal * 100
            })
            .sum()
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut patterns = Self::read_pattern_list(lines);
        patterns
            .iter_mut()
            .map(|pattern| {
                let (original_vertical_reflection, original_horizontal_reflection) =
                    pattern.summarize(pattern.grid.len() as u32, pattern.grid[0].len() as u32);
                for i in 0..pattern.grid.len() {
                    for j in 0..pattern.grid[0].len() {
                        pattern.grid[i][j] = if pattern.grid[i][j] == '#' { '.' } else { '#' };
                        let (new_vertical_reflection, new_horizontal_reflection) = pattern
                            .summarize(
                                original_horizontal_reflection,
                                original_vertical_reflection,
                            );
                        if new_horizontal_reflection != 0 || new_vertical_reflection != 0 {
                            return new_horizontal_reflection * 100 + new_vertical_reflection;
                        }
                        pattern.grid[i][j] = if pattern.grid[i][j] == '#' { '.' } else { '#' };
                    }
                }
                panic!("Cannot find valid smudge :(");
            })
            .sum()
    }
}
