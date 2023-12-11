use std::collections::{HashMap, HashSet, VecDeque};

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day10;

struct Graph<'a> {
    grid: &'a Vec<Vec<char>>,
    adj_list: HashMap<GridPos, Vec<GridPos>>,
    loop_points: HashSet<GridPos>,
    top_left: GridPos,
    bottom_right: GridPos,
}

const DX: &[i32] = &[0, 1, 0, -1];
const DY: &[i32] = &[-1, 0, 1, 0];

type GridPos = (usize, usize);

trait GridPosTraits {
    fn above(&self, other: &GridPos) -> bool;
    fn below(&self, other: &GridPos) -> bool;
    fn left_of(&self, other: &GridPos) -> bool;
    fn right_of(&self, other: &GridPos) -> bool;
    fn go_by_dir_vector(&self, dir_vector: (i32, i32)) -> Option<GridPos>;
    fn get_neighbors(&self, top_left: &GridPos, bottom_right: &GridPos) -> Vec<GridPos>;
}

impl GridPosTraits for GridPos {
    fn above(&self, other: &GridPos) -> bool {
        other.1 == self.1 && other.0 == self.0 + 1
    }
    fn below(&self, other: &GridPos) -> bool {
        other.1 == self.1 && other.0 == self.0 - 1
    }
    fn left_of(&self, other: &GridPos) -> bool {
        other.0 == self.0 && other.1 == self.1 + 1
    }
    fn right_of(&self, other: &GridPos) -> bool {
        other.0 == self.0 && other.1 == self.1 - 1
    }
    fn go_by_dir_vector(&self, dir_vector: (i32, i32)) -> Option<GridPos> {
        let nx = self.0.checked_add_signed(dir_vector.0.try_into().unwrap());
        let ny = self.1.checked_add_signed(dir_vector.1.try_into().unwrap());
        if nx.is_none() || ny.is_none() {
            None
        } else {
            Some((nx.unwrap(), ny.unwrap()))
        }
    }
    fn get_neighbors(&self, top_left: &GridPos, bottom_right: &GridPos) -> Vec<GridPos> {
        (0..4)
            .flat_map(|k| self.go_by_dir_vector((DX[k], DY[k])))
            .filter(|pos| {
                pos.0 >= top_left.0
                    && pos.0 <= bottom_right.0
                    && pos.1 >= top_left.1
                    && pos.1 <= bottom_right.1
            })
            .collect::<Vec<_>>()
    }
}

impl<'a> Graph<'a> {
    fn new(grid: &'a Vec<Vec<char>>) -> Graph<'a> {
        let top_left: GridPos = (0, 0);
        let bottom_right: GridPos = (grid.len() - 1, grid[0].len() - 1);
        Graph {
            grid,
            adj_list: HashMap::new(),
            loop_points: HashSet::new(),
            top_left,
            bottom_right,
        }
    }
    /// Check if pipe at grid_pos is linked pipe at parent_pos
    fn is_connected(&self, grid_pos: &GridPos, parent_pos: &GridPos) -> bool {
        match self.grid[grid_pos.0][grid_pos.1] {
            '|' => grid_pos.below(&parent_pos) || grid_pos.above(&parent_pos),
            '-' => grid_pos.left_of(&parent_pos) || grid_pos.right_of(&parent_pos),
            'L' => grid_pos.below(&parent_pos) || grid_pos.left_of(&parent_pos),
            'J' => grid_pos.below(&parent_pos) || grid_pos.right_of(&parent_pos),
            '7' => grid_pos.right_of(&parent_pos) || grid_pos.above(&parent_pos),
            'F' => grid_pos.above(&parent_pos) || grid_pos.left_of(&parent_pos),
            '.' => false,
            'S' => true,
            _ => {
                panic!(
                    "Invalid character found: {}",
                    self.grid[grid_pos.0][grid_pos.1]
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
        for neighbor in start_pos.get_neighbors(&self.top_left, &self.bottom_right) {
            if self.is_connected(&neighbor, &start_pos) && self.is_connected(&start_pos, &neighbor)
            {
                self.add_edge(&start_pos, &neighbor);
                if !self.loop_points.contains(&neighbor) {
                    self.mark_loop(neighbor);
                }
            }
        }
    }
    fn furthest_from(&self, start_pos: GridPos) -> u32 {
        let mut dist: HashMap<GridPos, u32> = HashMap::new();
        dist.insert(start_pos.clone(), 0);
        let mut q: VecDeque<GridPos> = VecDeque::new();
        q.push_back(start_pos);
        while let Some(curr) = q.pop_front() {
            for neighbor in self.adj_list.get(&curr).unwrap().iter() {
                if dist.get(neighbor).unwrap_or(&u32::MAX) > &(dist.get(&curr).unwrap() + 1) {
                    dist.insert(*neighbor, dist.get(&curr).unwrap() + 1);
                    q.push_back(*neighbor);
                }
            }
        }
        dist.iter().fold(0, |acc, curr| acc.max(*curr.1))
    }
    fn calculate_loop_area(&self) -> u32 {
        let top_left = self
            .loop_points
            .iter()
            .fold(&(usize::MAX, usize::MAX), |acc, curr| {
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
        (pos_list
            .iter()
            .enumerate()
            .fold(0i32, |acc, curr| {
                let p = if curr.0 == 0 {
                    pos_list.last().unwrap()
                } else {
                    &pos_list[curr.0 - 1]
                };
                acc + (p.0 as i32 - curr.1 .0 as i32) * (p.1 as i32 + curr.1 .1 as i32)
            })
            .abs()
            / 2) as u32
    }
}

impl Solution<u32, u32> for Day10 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let grid = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut start_pos: (usize, usize) = (grid.len(), grid[0].len());
        let mut graph = Graph::new(&grid);
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'S' {
                    start_pos = (i, j);
                }
            }
        }
        graph.mark_loop(start_pos);
        graph.furthest_from(start_pos)
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let grid = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut start_pos: (usize, usize) = (grid.len(), grid[0].len());
        let mut graph = Graph::new(&grid);
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'S' {
                    start_pos = (i, j);
                }
            }
        }
        graph.mark_loop(start_pos);
        graph.calculate_loop_area() - (graph.loop_points.len() as u32) / 2 + 1
    }
}
