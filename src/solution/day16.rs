use std::collections::VecDeque;

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day16;

const DIR_VECTOR: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];
const DIR_UP: usize = 0;
const DIR_RIGHT: usize = 1;
const DIR_DOWN: usize = 2;
const DIR_LEFT: usize = 3;

struct Grid {
    g: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Grid {
        Grid { g: grid }
    }
    fn rotate_clockwise(dir_index: usize) -> usize {
        (dir_index + 1) % 4
    }
    fn rotate_counterclockwise(dir_index: usize) -> usize {
        (dir_index + 3) % 4
    }
    fn generate_neighbors(cell_type: char, dir: usize) -> Vec<usize> {
        match cell_type {
            '|' => {
                if dir == DIR_UP || dir == DIR_DOWN {
                    vec![dir]
                } else {
                    vec![
                        Self::rotate_counterclockwise(dir),
                        Self::rotate_clockwise(dir),
                    ]
                }
            }
            '-' => {
                if dir == DIR_LEFT || dir == DIR_RIGHT {
                    vec![dir]
                } else {
                    vec![
                        Self::rotate_counterclockwise(dir),
                        Self::rotate_clockwise(dir),
                    ]
                }
            }
            '/' => {
                if dir == DIR_RIGHT || dir == DIR_LEFT {
                    vec![Self::rotate_counterclockwise(dir)]
                } else {
                    vec![Self::rotate_clockwise(dir)]
                }
            }
            '\\' => {
                if dir == DIR_DOWN || dir == DIR_UP {
                    vec![Self::rotate_counterclockwise(dir)]
                } else {
                    vec![Self::rotate_clockwise(dir)]
                }
            }
            '.' => {
                vec![dir]
            }
            _ => {
                panic!("Invalid symbol detected: {}", cell_type);
            }
        }
    }
    fn count_num_energized(
        // grid: &Vec<Vec<char>>,
        &self,
        start_pos: (usize, usize),
        start_dir: usize,
    ) -> u32 {
        let mut visited: Vec<Vec<Vec<bool>>> =
            vec![vec![vec![false; 4]; self.g[0].len()]; self.g.len()];
        let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
        q.push_back((start_pos.0, start_pos.1, start_dir));
        visited[start_pos.0][start_pos.1][start_dir] = true;
        while let Some((x, y, dir)) = q.pop_front() {
            for neighbor in Self::generate_neighbors(self.g[x][y], dir)
                .into_iter()
                .flat_map(|new_dir| {
                    let nx = x.checked_add_signed(DIR_VECTOR[new_dir].0 as isize);
                    let ny = y.checked_add_signed(DIR_VECTOR[new_dir].1 as isize);
                    if nx.is_none() || ny.is_none() {
                        None
                    } else {
                        Some((nx.unwrap(), ny.unwrap(), new_dir))
                    }
                })
            {
                if neighbor.0 < self.g.len()
                    && neighbor.1 < self.g[0].len()
                    && !visited[neighbor.0][neighbor.1][neighbor.2]
                {
                    q.push_back(neighbor);
                    visited[neighbor.0][neighbor.1][neighbor.2] = true;
                }
            }
        }
        let mut ret = 0;
        for i in 0..self.g.len() {
            for j in 0..self.g[i].len() {
                if visited[i][j].iter().any(|x| *x) {
                    ret += 1;
                }
            }
        }
        ret
    }
}

impl Day16 {
    fn parse_grid<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Vec<char>> {
        lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }
}

impl Solution<u32, u32> for Day16 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let grid = Grid::new(Self::parse_grid(lines));
        grid.count_num_energized((0, 0), DIR_RIGHT)
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let grid = Grid::new(Self::parse_grid(lines));
        (0..grid.g.len())
            .fold(0, |acc, i| {
                acc.max(grid.count_num_energized((i, 0), DIR_RIGHT))
                    .max(grid.count_num_energized((i, grid.g[0].len() - 1), DIR_LEFT))
            })
            .max((0..grid.g[0].len()).fold(0, |acc, i| {
                acc.max(grid.count_num_energized((0, i), DIR_DOWN))
                    .max(grid.count_num_energized((grid.g.len() - 1, i), DIR_UP))
            }))
    }
}
