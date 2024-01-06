use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;

use crate::solution::Solution;
use macros::return_type;

#[return_type(p1 = u32, p2 = u64)]
pub struct Day21;

const DX: &[isize] = &[-1, 1, 0, 0];
const DY: &[isize] = &[0, 0, -1, 1];

struct Grid {
    g: Vec<Vec<char>>,
    start_pos_by_delta: HashMap<(isize, isize), Vec<(isize, isize)>>,
    time_by_delta: HashMap<(isize, isize), (u64, u64)>,
    delta_step: u64,
    saturated_state: (u64, u64),
}

impl Grid {
    fn new(g: Vec<Vec<char>>) -> Grid {
        let mut start_pos = (g.len() as isize, g[0].len() as isize);
        for i in 0..g.len() {
            for j in 0..g[i].len() {
                if g[i][j] == 'S' {
                    start_pos = (i as isize, j as isize);
                }
            }
        }
        let mut new_grid = Grid {
            g,
            start_pos_by_delta: HashMap::new(),
            time_by_delta: HashMap::new(),
            delta_step: 0,
            saturated_state: (0, 0),
        };
        new_grid.traverse_mini_extended_grid(start_pos);
        new_grid.calculate_saturated_state(start_pos);
        new_grid
    }
    fn calculate_saturated_state(&mut self, start_pos: (isize, isize)) {
        let mut current_state = vec![start_pos];
        let mut num_elements = vec![1usize];
        for _ in 0..300 {
            let mut next_state = Vec::new();
            let mut seen = HashSet::new();
            for (x, y) in current_state.iter() {
                for (dx, dy) in iter::zip(DX, DY) {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0
                        || nx >= (self.g.len() as isize)
                        || ny < 0
                        || ny >= (self.g[0].len() as isize)
                    {
                        continue;
                    }
                    let tx = (nx + (self.g.len() * 4000) as isize) % (self.g.len() as isize);
                    let ty = (ny + (self.g[0].len() * 4000) as isize) % (self.g[0].len() as isize);
                    if self.g[tx as usize][ty as usize] != '#' && !seen.contains(&(nx, ny)) {
                        next_state.push((nx, ny));
                        seen.insert((nx, ny));
                    }
                }
            }
            current_state = next_state;
            num_elements.push(current_state.len());
        }
        self.saturated_state = (
            num_elements[num_elements.len() - 2] as u64,
            num_elements[num_elements.len() - 1] as u64,
        );
    }
    fn num_reachable_with_num_steps(
        &self,
        mut current_state: Vec<(isize, isize)>,
        num_steps: u64,
    ) -> u64 {
        for _ in 0..num_steps {
            let mut next_state = Vec::new();
            let mut seen = HashSet::new();
            for (x, y) in current_state.iter() {
                for (dx, dy) in iter::zip(DX, DY) {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0
                        || nx >= (self.g.len() as isize)
                        || ny < 0
                        || ny >= (self.g[0].len() as isize)
                    {
                        continue;
                    }
                    let tx = (nx + (self.g.len() * 4000) as isize) % (self.g.len() as isize);
                    let ty = (ny + (self.g[0].len() * 4000) as isize) % (self.g[0].len() as isize);
                    if self.g[tx as usize][ty as usize] != '#' && !seen.contains(&(nx, ny)) {
                        next_state.push((nx, ny));
                        seen.insert((nx, ny));
                    }
                }
            }
            current_state = next_state;
        }
        current_state.len() as u64
    }
    fn traverse_mini_extended_grid(&mut self, start_pos: (isize, isize)) {
        let mut iter = 0u32;
        let mut current_state = vec![start_pos];
        let mut dist = HashMap::new();
        dist.insert(start_pos, 0);
        while !current_state.is_empty() {
            let mut next_state = Vec::new();
            for (x, y) in current_state.iter() {
                for (dx, dy) in iter::zip(DX, DY) {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < -2 * (self.g.len() as isize)
                        || nx >= 3 * (self.g.len() as isize)
                        || ny < -2 * (self.g[0].len() as isize)
                        || ny >= 3 * (self.g[0].len() as isize)
                    {
                        continue;
                    }
                    let tx = (nx + (self.g.len() * 4000) as isize) % (self.g.len() as isize);
                    let ty = (ny + (self.g[0].len() * 4000) as isize) % (self.g[0].len() as isize);
                    if self.g[tx as usize][ty as usize] != '#' && !dist.contains_key(&(nx, ny)) {
                        dist.insert((nx, ny), iter + 1);
                        next_state.push((nx, ny));
                    }
                }
            }
            current_state = next_state;
            iter += 1;
        }
        for (k, v) in dist.iter() {
            for dx in -2..=2 {
                for dy in -2..=2 {
                    let nx = k.0 - dx * (self.g.len() as isize);
                    let ny = k.1 - dy * (self.g[0].len() as isize);
                    if nx < 0
                        || nx >= self.g.len() as isize
                        || ny < 0
                        || ny >= self.g[0].len() as isize
                    {
                        continue;
                    }
                    if !self.time_by_delta.contains_key(&(dx, dy)) {
                        self.time_by_delta
                            .insert((dx, dy), (v.to_owned() as u64, v.to_owned() as u64));
                        self.start_pos_by_delta.insert(
                            (dx, dy),
                            vec![(
                                (k.0 + 4000 * self.g.len() as isize) % self.g.len() as isize,
                                (k.1 + 4000 * self.g[0].len() as isize) % self.g[0].len() as isize,
                            )],
                        );
                    } else {
                        let curr_data = self.time_by_delta.get(&(dx, dy)).unwrap().to_owned();
                        if v < &(curr_data.0 as u32) {
                            self.start_pos_by_delta.insert(
                                (dx, dy),
                                vec![(
                                    (k.0 + 4000 * self.g.len() as isize) % self.g.len() as isize,
                                    (k.1 + 4000 * self.g[0].len() as isize)
                                        % self.g[0].len() as isize,
                                )],
                            );
                        } else if v == &(curr_data.0 as u32) {
                            self.start_pos_by_delta.get_mut(&(dx, dy)).unwrap().push((
                                (k.0 + 4000 * self.g.len() as isize) % self.g.len() as isize,
                                (k.1 + 4000 * self.g[0].len() as isize) % self.g[0].len() as isize,
                            ));
                        }
                        self.time_by_delta.insert(
                            (dx, dy),
                            (
                                curr_data.0.min(v.to_owned() as u64),
                                curr_data.1.max(v.to_owned() as u64),
                            ),
                        );
                    }
                }
            }
        }
        self.delta_step =
            self.time_by_delta.get(&(2, 0)).unwrap().0 - self.time_by_delta.get(&(1, 0)).unwrap().0;
    }
    fn extrapolate_2d(&self, start_pos: (isize, isize), max_num_steps: u64) -> u64 {
        let mut low = 1u64;
        let mut high = 1000000000u64;
        let mut lb = high;
        // find first position such that max time >= num_step
        while low <= high {
            let mid = low + (high - low) / 2;
            let nrb = self.time_by_delta.get(&start_pos).unwrap().1 + self.delta_step * mid
                - self.delta_step;
            if nrb >= max_num_steps {
                lb = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        low = 1u64;
        high = 1000000000u64;
        let mut rb = 0u64;
        // find first position such that min time > num_step
        while low <= high {
            let mid = low + (high - low) / 2;
            let nlb = self.time_by_delta.get(&start_pos).unwrap().0 + self.delta_step * mid
                - self.delta_step;
            if nlb >= max_num_steps {
                rb = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        let sum_all = lb * (lb - 1) / 2;
        let sum_even = ((lb - 1) / 2) * ((lb - 1) / 2 + 1);
        let sum_odd = sum_all - sum_even;
        let mut ret = if max_num_steps % 2 == 1 {
            self.saturated_state.0 * sum_odd + self.saturated_state.1 * sum_even
        } else {
            self.saturated_state.1 * sum_odd + self.saturated_state.0 * sum_even
        };
        while lb < rb {
            let remain_steps = max_num_steps + self.delta_step
                - self.time_by_delta.get(&start_pos).unwrap().0
                - self.delta_step * lb;
            ret += lb
                * self.num_reachable_with_num_steps(
                    self.start_pos_by_delta
                        .get(&(start_pos.0 * 2, start_pos.1 * 2))
                        .unwrap()
                        .clone(),
                    remain_steps,
                );
            lb += 1;
        }
        ret
    }
    fn extrapolate_1d(&self, start_pos: (isize, isize), max_num_steps: u64) -> u64 {
        let mut low = 1u64;
        let mut high = 1000000000u64;
        let mut lb = high;
        // find first position such that max time >= num_step
        while low <= high {
            let mid = low + (high - low) / 2;
            let nrb = self.time_by_delta.get(&start_pos).unwrap().1 + self.delta_step * mid
                - self.delta_step;
            if nrb >= max_num_steps {
                lb = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        low = 1u64;
        high = 1000000000u64;
        let mut rb = 0u64;
        // find first position such that min time > num_step
        while low <= high {
            let mid = low + (high - low) / 2;
            let nlb = self.time_by_delta.get(&start_pos).unwrap().0 + self.delta_step * mid
                - self.delta_step;
            if nlb >= max_num_steps {
                rb = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        let mut ret = ((lb - 1) / 2) * (self.saturated_state.1 + self.saturated_state.0)
            + ((lb - 1) % 2)
                * (if max_num_steps % 2 == 1 {
                    self.saturated_state.1
                } else {
                    self.saturated_state.0
                });
        while lb < rb {
            let curr_step = max_num_steps + self.delta_step
                - self.time_by_delta.get(&start_pos).unwrap().0
                - self.delta_step * lb;
            ret += self.num_reachable_with_num_steps(
                self.start_pos_by_delta
                    .get(&(start_pos.0 * 2, start_pos.1 * 2))
                    .unwrap()
                    .clone(),
                curr_step,
            );
            lb += 1;
        }
        ret
    }
}

impl Solution<u32, u64> for Day21 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let grid = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut start_pos = (grid.len(), grid[0].len());
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'S' {
                    start_pos = (i, j);
                }
            }
        }
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        q.push_back(start_pos);
        visited[start_pos.0][start_pos.1] = true;
        for _iter in 0..64 {
            q.push_back((grid.len(), grid[0].len()));
            let mut reached: Vec<(usize, usize)> = Vec::new();
            while let Some((x, y)) = q.pop_front() {
                if x == grid.len() {
                    break;
                }
                for (dx, dy) in iter::zip(DX, DY) {
                    let nx = x.checked_add_signed(*dx as isize);
                    let ny = y.checked_add_signed(*dy as isize);
                    if nx.is_none()
                        || ny.is_none()
                        || nx.unwrap() >= grid.len()
                        || ny.unwrap() >= grid[0].len()
                    {
                        continue;
                    }
                    if !visited[nx.unwrap()][ny.unwrap()] && grid[nx.unwrap()][ny.unwrap()] != '#' {
                        visited[nx.unwrap()][ny.unwrap()] = true;
                        q.push_back((nx.unwrap(), ny.unwrap()));
                        reached.push((nx.unwrap(), ny.unwrap()));
                    }
                }
            }
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    visited[i][j] = false;
                }
            }
            for (x, y) in reached.into_iter() {
                visited[x][y] = true;
            }
        }
        q.len() as u32
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
        let max_num_steps = 26501365;
        let g = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let grid = Grid::new(g);

        let mut ans = if max_num_steps % 2 == 0 {
            grid.saturated_state.1
        } else {
            grid.saturated_state.0
        };
        let corners = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
        let edges = vec![(-1, 0), (0, 1), (0, -1), (1, 0)];

        for corner in corners {
            let result = grid.extrapolate_2d(corner, max_num_steps);
            ans += result;
        }
        for edge in edges {
            let result = grid.extrapolate_1d(edge, max_num_steps);
            ans += result;
        }
        ans
    }
}
