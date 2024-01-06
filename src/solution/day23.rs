use crate::solution::Solution;
use macros::return_type;

#[return_type(p1 = i32, p2 = i32)]
pub struct Day23;

struct Graph {
    grid: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
}

const STATE: &[(isize, isize, char)] = &[(-1, 0, '^'), (0, 1, '>'), (0, -1, '<'), (1, 0, 'v')];
const MAX_DEPTH_LIMIT: i32 = 7000;

impl Graph {
    fn new(grid: Vec<Vec<char>>) -> Graph {
        Graph {
            visited: vec![vec![false; grid[0].len()]; grid.len()],
            grid,
        }
    }
    fn find_longest_path(&mut self, curr_pos: (isize, isize), curr_dist: i32) -> i32 {
        if curr_pos.0 == self.grid.len() as isize - 1 {
            return curr_dist;
        }
        self.visited[curr_pos.0 as usize][curr_pos.1 as usize] = true;
        let mut ret = -1000000000;
        for (dx, dy, c) in STATE.iter() {
            if c == &self.grid[curr_pos.0 as usize][curr_pos.1 as usize]
                || self.grid[curr_pos.0 as usize][curr_pos.1 as usize] == '.'
            {
                let nx = dx + curr_pos.0;
                let ny = dy + curr_pos.1;
                if nx < 0
                    || nx >= self.grid.len() as isize
                    || ny < 0
                    || ny >= self.grid[0].len() as isize
                {
                    continue;
                }
                if !self.visited[nx as usize][ny as usize]
                    && self.grid[nx as usize][ny as usize] != '#'
                {
                    ret = ret.max(self.find_longest_path((nx, ny), curr_dist + 1));
                }
            }
        }
        self.visited[curr_pos.0 as usize][curr_pos.1 as usize] = false;
        ret
    }
    fn find_longest_path_optimized(
        &mut self,
        start: (isize, isize),
        end: (isize, isize),
        curr_dist: i32,
    ) -> i32 {
        if start == end {
            return curr_dist * 2;
        }
        if curr_dist * 2 >= MAX_DEPTH_LIMIT {
            return -1000000000;
        }
        let mut ret = -1000000000;
        for (dx, dy, _) in STATE.iter() {
            let nx = start.0 + dx;
            let ny = start.1 + dy;
            if nx == end.0 && ny == end.1 {
                ret = ret.max(curr_dist * 2 + 1);
            }
        }
        self.visited[start.0 as usize][start.1 as usize] = true;
        self.visited[end.0 as usize][end.1 as usize] = true;
        let mut start_neighbor = Vec::new();
        let mut end_neighbor = Vec::new();
        for (dx, dy, _) in STATE.iter() {
            let nx = dx + start.0;
            let ny = dy + start.1;
            if nx < 0
                || nx >= self.grid.len() as isize
                || ny < 0
                || ny >= self.grid[0].len() as isize
            {
                continue;
            }
            if !self.visited[nx as usize][ny as usize] && self.grid[nx as usize][ny as usize] != '#'
            {
                start_neighbor.push((nx, ny));
            }
        }
        for (dx, dy, _) in STATE.iter() {
            let nx = dx + end.0;
            let ny = dy + end.1;
            if nx < 0
                || nx >= self.grid.len() as isize
                || ny < 0
                || ny >= self.grid[0].len() as isize
            {
                continue;
            }
            if !self.visited[nx as usize][ny as usize] && self.grid[nx as usize][ny as usize] != '#'
            {
                end_neighbor.push((nx, ny));
            }
        }
        for px in start_neighbor.iter() {
            for py in end_neighbor.iter() {
                ret = ret.max(self.find_longest_path_optimized(*px, *py, curr_dist + 1));
            }
        }
        self.visited[start.0 as usize][start.1 as usize] = false;
        self.visited[end.0 as usize][end.1 as usize] = false;
        ret
    }
}

impl Solution<i32, i32> for Day23 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> i32 {
        let grid = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut graph = Graph::new(grid);
        for i in 0..graph.grid[0].len() {
            if graph.grid[0][i] == '.' {
                return graph.find_longest_path((0, i as isize), 0);
            }
        }
        panic!("something is wrong")
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> i32 {
        let grid = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut start: (isize, isize) = (0, 0);
        let mut end: (isize, isize) = (0, 0);
        for j in 0..grid[0].len() {
            if grid[0][j] == '.' {
                start = (0, j as isize);
            }
            if grid.last().unwrap()[j] == '.' {
                end = (grid.len() as isize - 1, j as isize);
            }
        }
        let mut graph = Graph::new(grid);
        graph.find_longest_path_optimized(start, end, 0)
    }
}
