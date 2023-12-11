use std::collections::HashSet;

use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u64)]
pub struct Day11;

impl Day11 {
    fn rescale_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let duplicated_rows: HashSet<usize> = HashSet::from_iter(
            (0..grid.len())
                .filter(|i| (0..grid[*i].len()).filter(|j| grid[*i][*j] == '#').count() == 0),
        );
        let duplicated_cols: HashSet<usize> = HashSet::from_iter(
            (0..grid[0].len())
                .filter(|j| (0..grid.len()).filter(|i| grid[*i][*j] == '#').count() == 0),
        );
        let mut ret: Vec<Vec<char>> = Vec::new();
        for i in 0..grid.len() {
            let iter_cnt = if duplicated_rows.contains(&i) { 2 } else { 1 };
            for _ in 0..iter_cnt {
                ret.push(Vec::new());
                for j in 0..grid[i].len() {
                    ret.last_mut().unwrap().push(grid[i][j]);
                    if duplicated_cols.contains(&j) {
                        ret.last_mut().unwrap().push(grid[i][j]);
                    }
                }
            }
        }
        ret
    }
}

impl Solution<u32, u64> for Day11 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let grid = lines
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rescaled_grid = Self::rescale_grid(&grid);
        let mut ret = 0;
        let mut galaxy_pos: Vec<(usize, usize)> = Vec::new();
        for i in 0..rescaled_grid.len() {
            for j in 0..rescaled_grid[i].len() {
                if rescaled_grid[i][j] == '#' {
                    galaxy_pos.push((i, j));
                }
            }
        }
        for i in 0..galaxy_pos.len() {
            for j in i..galaxy_pos.len() {
                ret += galaxy_pos[i].0.abs_diff(galaxy_pos[j].0) as u32
                    + galaxy_pos[i].1.abs_diff(galaxy_pos[j].1) as u32;
            }
        }
        ret
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
        let grid = lines
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let duplicated_rows: HashSet<usize> = HashSet::from_iter(
            (0..grid.len())
                .filter(|i| (0..grid[*i].len()).filter(|j| grid[*i][*j] == '#').count() == 0),
        );
        let duplicated_cols: HashSet<usize> = HashSet::from_iter(
            (0..grid[0].len())
                .filter(|j| (0..grid.len()).filter(|i| grid[*i][*j] == '#').count() == 0),
        );
        let mut count_by_rows = vec![0; grid.len()];
        let mut count_by_cols = vec![0; grid[0].len()];
        let mut ret = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '#' {
                    count_by_rows[i] += 1;
                    count_by_cols[j] += 1;
                }
            }
        }
        for i in 0..grid.len() {
            let mut total_cost: u64 = (0..=i).map(|index| if duplicated_rows.contains(&index) { 1000000u64 } else { 1u64 }).sum();
            for j in 0..=i {
                total_cost -= if duplicated_rows.contains(&j) { 1000000u64 } else { 1 };
                if count_by_rows[j] > 0 {
                    ret += total_cost * count_by_rows[j] * count_by_rows[i];
                }
            }
        }
        for j in 0..grid[0].len() {
            let mut total_cost: u64 = (0..=j).map(|index| if duplicated_cols.contains(&index) { 1000000u64 } else { 1u64 }).sum();
            for i in 0..=j {
                total_cost -= if duplicated_cols.contains(&i) { 1000000u64 } else { 1u64 };
                if count_by_cols[i] > 0 {
                    ret += total_cost * count_by_cols[j] * count_by_cols[i];
                }
            }
        }
        ret
    }
}
