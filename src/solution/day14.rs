use macros::return_type;

use crate::solution::Solution;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day14;

impl Day14 {
    fn roll_north(grid: &mut Vec<Vec<char>>) {
        for j in 0..grid[0].len() {
            for i in 1..grid.len() {
                if grid[i][j] == 'O' {
                    let mut fillable_index = i - 1;
                    loop {
                        if grid[fillable_index][j] != '.' {
                            fillable_index += 1;
                            break;
                        }
                        if fillable_index == 0 {
                            break;
                        }
                        fillable_index -= 1;
                    }
                    grid[i][j] = '.';
                    grid[fillable_index][j] = 'O';
                }
            }
        }
    }
    fn roll_west(grid: &mut Vec<Vec<char>>) {
        for i in 0..grid.len() {
            for j in 1..grid[0].len() {
                if grid[i][j] == 'O' {
                    let mut fillable_index = j - 1;
                    loop {
                        if grid[i][fillable_index] != '.' {
                            fillable_index += 1;
                            break;
                        }
                        if fillable_index == 0 {
                            break;
                        }
                        fillable_index -= 1;
                    }
                    grid[i][j] = '.';
                    grid[i][fillable_index] = 'O';
                }
            }
        }
    }

    fn roll_south(grid: &mut Vec<Vec<char>>) {
        for j in 0..grid[0].len() {
            for i in (0..grid.len() - 1).rev() {
                if grid[i][j] == 'O' {
                    let mut fillable_index = i + 1;
                    loop {
                        if grid[fillable_index][j] != '.' {
                            fillable_index -= 1;
                            break;
                        }
                        if fillable_index == grid.len() - 1 {
                            break;
                        }
                        fillable_index += 1;
                    }
                    grid[i][j] = '.';
                    grid[fillable_index][j] = 'O';
                }
            }
        }
    }

    fn roll_east(grid: &mut Vec<Vec<char>>) {
        for i in 0..grid.len() {
            for j in (0..grid[0].len() - 1).rev() {
                if grid[i][j] == 'O' {
                    let mut fillable_index = j + 1;
                    loop {
                        if grid[i][fillable_index] != '.' {
                            fillable_index -= 1;
                            break;
                        }
                        if fillable_index == grid[0].len() - 1 {
                            break;
                        }
                        fillable_index += 1;
                    }
                    grid[i][j] = '.';
                    grid[i][fillable_index] = 'O';
                }
            }
        }
    }

    fn calculate_weight(grid: &Vec<Vec<char>>) -> u32 {
        (0..grid.len())
            .map(|i| grid[i].iter().filter(|x| *x == &'O').count() * (grid.len() - i))
            .sum::<usize>() as u32
    }
    fn rotate_cycle(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        Self::roll_north(&mut grid);
        Self::roll_west(&mut grid);
        Self::roll_south(&mut grid);
        Self::roll_east(&mut grid);
        grid
    }
    fn is_same_grid(g1: &Vec<Vec<char>>, g2: &Vec<Vec<char>>) -> bool {
        (0..g1.len()).all(|i| (0..g1[i].len()).all(|j| g1[i][j] == g2[i][j]))
    }
}

impl Solution<u32, u32> for Day14 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut grid = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self::roll_north(&mut grid);
        Self::calculate_weight(&grid)
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut grid_list = vec![lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()];
        let target_num_cycle = 1000000000;
        loop {
            let new_grid = Self::rotate_cycle(grid_list.last().unwrap().clone());
            for i in 0..grid_list.len() {
                if Self::is_same_grid(&new_grid, &grid_list[i]) {
                    return Self::calculate_weight(
                        &grid_list[(target_num_cycle - i) % (grid_list.len() - i) + i],
                    );
                }
            }
            grid_list.push(new_grid);
        }
    }
}
