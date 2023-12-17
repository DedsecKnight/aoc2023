use crate::solution::Solution;
use macros::return_type;
use std::collections::{BTreeSet, HashMap};

#[return_type(p1 = u32, p2 = u32)]
pub struct Day17;

type State = (usize, usize, usize, u32);

const DIR_VECTOR: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

struct Grid {
    g: Vec<Vec<u32>>,
}

impl Grid {
    fn new(grid: Vec<Vec<u32>>) -> Grid {
        Grid { g: grid }
    }
    fn find_best_path(&self, min_step_before_turn: u32, max_step_before_turn: u32) -> u32 {
        let mut dist: HashMap<State, u32> = HashMap::new();
        let mut q: BTreeSet<(u32, State)> = BTreeSet::new();
        for dir in 0..4 {
            let curr_state = (0, 0, dir, 0);
            q.insert((0, curr_state));
            dist.insert(curr_state, 0);
        }
        while let Some((curr_dist, curr_state)) = q.pop_first() {
            for k in 0..4 {
                if k == (curr_state.2 + 2) % 4 {
                    continue;
                }
                let nx = curr_state.0.checked_add_signed(DIR_VECTOR[k].0 as isize);
                let ny = curr_state.1.checked_add_signed(DIR_VECTOR[k].1 as isize);
                if nx.is_none()
                    || ny.is_none()
                    || nx.unwrap() >= self.g.len()
                    || ny.unwrap() >= self.g[0].len()
                {
                    continue;
                }
                if curr_state.3 < min_step_before_turn && k != curr_state.2 {
                    continue;
                }
                let ncounter = if k == curr_state.2 {
                    curr_state.3 + 1
                } else {
                    1
                };
                if ncounter > max_step_before_turn {
                    continue;
                }
                let nstate = (nx.unwrap(), ny.unwrap(), k, ncounter);
                let ndist = dist.get(&nstate).unwrap_or(&u32::MAX).to_owned();
                if ndist > curr_dist + self.g[nx.unwrap()][ny.unwrap()] {
                    q.remove(&(ndist, nstate));
                    dist.insert(nstate, curr_dist + self.g[nx.unwrap()][ny.unwrap()]);
                    q.insert((dist.get(&nstate).unwrap().to_owned(), nstate));
                }
            }
        }
        dist.iter()
            .filter(|elem| {
                elem.0 .0 == self.g.len() - 1
                    && elem.0 .1 == self.g[0].len() - 1
                    && elem.0 .3 >= min_step_before_turn
            })
            .fold(u32::MAX, |acc, curr| acc.min(*curr.1))
    }
}

impl Solution<u32, u32> for Day17 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let grid = Grid::new(
            lines
                .map(|line| {
                    line.chars()
                        .flat_map(|c| c.to_digit(10))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        );
        grid.find_best_path(0, 3)
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let grid = Grid::new(
            lines
                .map(|line| {
                    line.chars()
                        .flat_map(|c| c.to_digit(10))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        );
        grid.find_best_path(4, 10)
    }
}
