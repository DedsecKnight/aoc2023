use std::collections::{HashMap, VecDeque, HashSet};

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = i32)]
pub struct Day10;

struct Graph<'a> {
    grid: &'a Vec<Vec<char>>,
    adj_list: HashMap<Coordinate, Vec<Coordinate>>,
    dist: HashMap<GridPos, u32>,
    coords_grid_mapping: HashMap<Coordinate, GridPos>,
}

const DX: &[i32] = &[0, 1, 0, -1];
const DY: &[i32] = &[-1, 0, 1, 0];

type GridPos = (i32, i32);
type Coordinate = (i32, i32);

impl<'a> Graph<'a> {
    fn new(grid: &'a Vec<Vec<char>>) -> Graph<'a> {
        Graph {
            grid,
            adj_list: HashMap::new(),
            coords_grid_mapping: HashMap::new(),
            dist: HashMap::new(),
        }
    }
    // check if pos is linked with other
    fn is_connected(
        &self,
        grid_pos: GridPos,
        curr_coord: Coordinate,
        parent_coord: Coordinate,
    ) -> bool {
        match self.grid[grid_pos.0 as usize][grid_pos.1 as usize] {
            '|' => (curr_coord.0 - parent_coord.0).abs() == 1 && curr_coord.1 == parent_coord.1,
            '-' => (curr_coord.1 - parent_coord.1).abs() == 1 && curr_coord.0 == parent_coord.0,
            'L' => {
                ((curr_coord.0 - 1 == parent_coord.0) && parent_coord.1 == curr_coord.1)
                    || (curr_coord.0 == parent_coord.0 && curr_coord.1 + 1 == parent_coord.1)
            }
            'J' => {
                ((curr_coord.0 - 1 == parent_coord.0) && parent_coord.1 == curr_coord.1)
                    || (curr_coord.0 == parent_coord.0 && curr_coord.1 - 1 == parent_coord.1)
            }
            '7' => {
                ((curr_coord.0 + 1 == parent_coord.0) && parent_coord.1 == curr_coord.1)
                    || (curr_coord.0 == parent_coord.0 && curr_coord.1 - 1 == parent_coord.1)
            }
            'F' => {
                ((curr_coord.0 + 1 == parent_coord.0) && parent_coord.1 == curr_coord.1)
                    || (curr_coord.0 == parent_coord.0 && curr_coord.1 + 1 == parent_coord.1)
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
    fn add_edge(&mut self, from: &Coordinate, to: &Coordinate) {
        if !self.adj_list.contains_key(from) {
            self.adj_list.insert(*from, Vec::new());
        }
        self.adj_list.get_mut(from).unwrap().push(*to);
    }
    fn mark_loop(&mut self, start_pos: GridPos, coordinate: Coordinate) {
        // associate start_pos with coordinates
        self.coords_grid_mapping
            .insert(coordinate.clone(), start_pos.clone());
        self.dist.insert(coordinate.clone(), u32::MAX);
        for k in 0..4 {
            let (nx, ny) = (start_pos.0 + DX[k], start_pos.1 + DY[k]);
            if nx < 0 || nx >= self.grid.len() as i32 || ny < 0 || ny >= self.grid[0].len() as i32 {
                continue;
            }
            let next_coord = (coordinate.0 + DX[k], coordinate.1 + DY[k]);
            if self.is_connected((nx, ny), next_coord.clone(), coordinate.clone())
                && self.is_connected(start_pos, coordinate.clone(), next_coord.clone())
            {
                self.add_edge(&coordinate, &next_coord);
                if !self.dist.contains_key(&next_coord) {
                    self.mark_loop((nx, ny), next_coord);
                }
            }
        }
    }
    fn furthest_from(&mut self, start_pos: Coordinate) -> u32 {
        self.dist.insert(start_pos.clone(), 0);
        let mut q: VecDeque<Coordinate> = VecDeque::new();
        q.push_back(start_pos);
        while let Some(curr) = q.pop_front() {
            for neighbor in self.adj_list.get(&curr).unwrap().iter() {
                if *self.dist.get(neighbor).unwrap() > *self.dist.get(&curr).unwrap() + 1 {
                    self.dist
                        .insert(*neighbor, self.dist.get(&curr).unwrap() + 1);
                    q.push_back(*neighbor);
                }
            }
        }
        self.dist.iter().fold(0, |acc, curr| acc.max(*curr.1))
    }
}

impl Day10 {
    fn calculate_polygon_area(polygon: &Vec<(i32, i32)>) -> i32 {
        polygon.iter().enumerate().fold(0i32, |acc, curr| {
            let p = if curr.0 == 0 { polygon.last().unwrap() } else { &polygon[curr.0-1]};
            acc + (p.0 - curr.1.0) * (p.1 + curr.1.1)
        }).abs() / 2
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
        graph.mark_loop(start_pos, (0, 0));
        graph.furthest_from((0, 0))
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
        graph.mark_loop(start_pos, (0, 0));
        let border_elems = graph.coords_grid_mapping.iter().map(|x| (x.0.clone(), x.1.clone()) ).collect::<Vec<_>>();
        let top_left = border_elems.iter().fold(border_elems[0].0, |acc, curr| {
            if curr.0.0 < acc.0 {
                curr.0
            } else if curr.0.0 == acc.0 {
                if curr.0.1 < acc.1 {
                    curr.0
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
            for neighbor in graph.adj_list.get(&curr_pos).unwrap() {
                if !seen.contains(neighbor) {
                    seen.insert(*neighbor);
                    curr_pos = neighbor.clone();
                    break;
                }
            }
            if curr_pos == top_left {
                break;
            }
        }
        Self::calculate_polygon_area(&pos_list) - (pos_list.len() as i32) / 2 + 1
    }
}
