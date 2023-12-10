use std::collections::{HashMap, HashSet, VecDeque};

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = i32)]
pub struct Day10;

struct Graph<'a> {
    grid: &'a Vec<Vec<char>>,
    adj_list: HashMap<GridPos, Vec<GridPos>>,
    dist: HashMap<GridPos, u32>,
    loop_points: HashSet<GridPos>,
}

const DX: &[i32] = &[0, 1, 0, -1];
const DY: &[i32] = &[-1, 0, 1, 0];

type GridPos = (i32, i32);

impl<'a> Graph<'a> {
    fn new(grid: &'a Vec<Vec<char>>) -> Graph<'a> {
        Graph {
            grid,
            adj_list: HashMap::new(),
            loop_points: HashSet::new(),
            dist: HashMap::new(),
        }
    }
    /// Check if pipe at grid_pos is linked pipe at parent_pos
    fn is_connected(&self, grid_pos: GridPos, parent_pos: GridPos) -> bool {
        match self.grid[grid_pos.0 as usize][grid_pos.1 as usize] {
            '|' => (grid_pos.0 - parent_pos.0).abs() == 1 && grid_pos.1 == parent_pos.1,
            '-' => (grid_pos.1 - parent_pos.1).abs() == 1 && grid_pos.0 == parent_pos.0,
            'L' => {
                ((grid_pos.0 - 1 == parent_pos.0) && parent_pos.1 == grid_pos.1)
                    || (grid_pos.0 == parent_pos.0 && grid_pos.1 + 1 == parent_pos.1)
            }
            'J' => {
                ((grid_pos.0 - 1 == parent_pos.0) && parent_pos.1 == grid_pos.1)
                    || (grid_pos.0 == parent_pos.0 && grid_pos.1 - 1 == parent_pos.1)
            }
            '7' => {
                ((grid_pos.0 + 1 == parent_pos.0) && parent_pos.1 == grid_pos.1)
                    || (grid_pos.0 == parent_pos.0 && grid_pos.1 - 1 == parent_pos.1)
            }
            'F' => {
                ((grid_pos.0 + 1 == parent_pos.0) && parent_pos.1 == grid_pos.1)
                    || (grid_pos.0 == parent_pos.0 && grid_pos.1 + 1 == parent_pos.1)
            }
            '.' => false,
            'S' => true,
            _ => {
                panic!(
                    "Invalid character found: {}",
                    self.grid[grid_pos.0 as usize][grid_pos.1 as usize]
                )
            }
        }
    }
    fn add_edge(&mut self, from: &GridPos, to: &GridPos) {
        if !self.adj_list.contains_key(from) {
            self.adj_list.insert(*from, Vec::new());
        }
        self.adj_list.get_mut(from).unwrap().push(*to);
    }
    fn mark_loop(&mut self, start_pos: GridPos) {
        self.loop_points.insert(start_pos.clone());
        for k in 0..4 {
            let (nx, ny) = (start_pos.0 + DX[k], start_pos.1 + DY[k]);
            if nx < 0 || nx >= self.grid.len() as i32 || ny < 0 || ny >= self.grid[0].len() as i32 {
                continue;
            }
            if self.is_connected((nx, ny), start_pos.clone())
                && self.is_connected(start_pos.clone(), (nx, ny))
            {
                self.add_edge(&start_pos, &(nx, ny));
                if !self.loop_points.contains(&(nx, ny)) {
                    self.mark_loop((nx, ny));
                }
            }
        }
    }
    fn furthest_from(&mut self, start_pos: GridPos) -> u32 {
        self.dist.insert(start_pos.clone(), 0);
        let mut q: VecDeque<GridPos> = VecDeque::new();
        q.push_back(start_pos);
        while let Some(curr) = q.pop_front() {
            for neighbor in self.adj_list.get(&curr).unwrap().iter() {
                if self.dist.get(neighbor).unwrap_or(&u32::MAX)
                    > &(self.dist.get(&curr).unwrap() + 1)
                {
                    self.dist
                        .insert(*neighbor, self.dist.get(&curr).unwrap() + 1);
                    q.push_back(*neighbor);
                }
            }
        }
        self.dist.iter().fold(0, |acc, curr| acc.max(*curr.1))
    }
    /// Return sorted list of all points on pipe border in counter-clockwise order, starting from top-left point.
    fn sorted_loop_points(&self) -> Vec<GridPos> {
        let top_left = self
            .loop_points
            .iter()
            .fold(&(i32::MAX, i32::MAX), |acc, curr| {
                if curr.0 < acc.0 {
                    curr
                } else if curr.0 == acc.0 {
                    if curr.1 < acc.1 {
                        curr
                    } else {
                        acc
                    }
                } else {
                    acc
                }
            });
        let mut seen: HashSet<GridPos> = HashSet::new();
        let mut pos_list: Vec<GridPos> = Vec::new();
        let mut curr_pos = top_left;
        loop {
            pos_list.push(curr_pos.clone());
            for neighbor in self.adj_list.get(&curr_pos).unwrap() {
                if !seen.contains(neighbor) {
                    seen.insert(*neighbor);
                    curr_pos = neighbor;
                    break;
                }
            }
            if curr_pos == top_left {
                break;
            }
        }
        pos_list
    }
}

impl Day10 {
    fn calculate_loop_area(polygon: &Vec<(i32, i32)>) -> i32 {
        polygon
            .iter()
            .enumerate()
            .fold(0i32, |acc, curr| {
                let p = if curr.0 == 0 {
                    polygon.last().unwrap()
                } else {
                    &polygon[curr.0 - 1]
                };
                acc + (p.0 - curr.1 .0) * (p.1 + curr.1 .1)
            })
            .abs()
            / 2
    }
}

impl Solution<u32, i32> for Day10 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let grid = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut start_pos: (i32, i32) = (
            grid.len().try_into().unwrap(),
            grid[0].len().try_into().unwrap(),
        );
        let mut graph = Graph::new(&grid);
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'S' {
                    start_pos = (i.try_into().unwrap(), j.try_into().unwrap());
                }
            }
        }
        graph.mark_loop(start_pos);
        graph.furthest_from(start_pos)
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> i32 {
        let grid = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut start_pos: (i32, i32) = (
            grid.len().try_into().unwrap(),
            grid[0].len().try_into().unwrap(),
        );
        let mut graph = Graph::new(&grid);
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'S' {
                    start_pos = (i.try_into().unwrap(), j.try_into().unwrap());
                }
            }
        }
        graph.mark_loop(start_pos);
        let sorted_points = graph.sorted_loop_points();
        Self::calculate_loop_area(&sorted_points) - (sorted_points.len() as i32) / 2 + 1
    }
}
