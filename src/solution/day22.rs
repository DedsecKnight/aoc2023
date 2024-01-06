use std::collections::{HashSet, VecDeque};

use crate::solution::Solution;
use macros::return_type;

#[return_type(p1 = u32, p2 = u32)]
pub struct Day22;

#[derive(Copy, Clone)]
struct Coordinate {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Copy, Clone)]
struct Block {
    lb: Coordinate,
    rb: Coordinate,
}

struct Space3D {
    space: Vec<Vec<Vec<isize>>>,
    blocks: Vec<Block>,
    graph: Vec<Vec<usize>>,
    reverse_graph: Vec<Vec<isize>>,
}

impl Block {
    fn new(raw_block: &str) -> Block {
        let parsed_coordinates = raw_block
            .split('~')
            .map(Coordinate::new)
            .collect::<Vec<_>>();
        assert!(parsed_coordinates.len() == 2);
        Block {
            lb: parsed_coordinates[0],
            rb: parsed_coordinates[1],
        }
    }
}

impl Coordinate {
    fn new(raw_coordinate: &str) -> Coordinate {
        let parsed_elem = raw_coordinate
            .split(',')
            .map(|elem| elem.parse().unwrap())
            .collect::<Vec<_>>();
        assert!(parsed_elem.len() == 3);
        Coordinate {
            x: parsed_elem[0],
            y: parsed_elem[1],
            z: parsed_elem[2] - 1,
        }
    }
}

impl Space3D {
    fn new(dim_size: usize) -> Space3D {
        Space3D {
            space: vec![vec![vec![-1; dim_size]; dim_size]; dim_size],
            blocks: Vec::new(),
            graph: Vec::new(),
            reverse_graph: Vec::new(),
        }
    }
    fn add_block(&mut self, block: &Block) {
        for x in block.lb.x..=block.rb.x {
            for y in block.lb.y..=block.rb.y {
                for z in block.lb.z..=block.rb.z {
                    self.space[x as usize][y as usize][z as usize] = self.blocks.len() as isize;
                }
            }
        }
        self.blocks.push(block.clone());
        self.graph.push(Vec::new());
        self.reverse_graph.push(Vec::new());
    }
    fn simulate_fall(&mut self, blocks: Vec<Block>) {
        let mut q = VecDeque::from(blocks);
        while let Some(mut block) = q.pop_front() {
            if block.lb.z == 0 {
                self.add_block(&block);
            } else {
                let mut fallable = true;
                for x in block.lb.x..=block.rb.x {
                    for y in block.lb.y..=block.rb.y {
                        if self.space[x as usize][y as usize][block.lb.z as usize - 1] != -1 {
                            fallable = false;
                        }
                    }
                }
                if !fallable {
                    self.add_block(&block);
                } else {
                    block.lb.z -= 1;
                    block.rb.z -= 1;
                    q.push_back(block);
                }
            }
        }
    }

    fn count_disintegratable_block(&mut self) -> u32 {
        let mut ret = 0;
        for i in 0..self.blocks.len() {
            if self.graph[i]
                .iter()
                .all(|neighbor| self.reverse_graph[*neighbor].len() >= 2)
            {
                ret += 1;
            }
        }
        ret
    }
    fn build_graph(&mut self) {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.lb.z == 0 {
                continue;
            }
            let mut seen = HashSet::new();
            for x in block.lb.x..=block.rb.x {
                for y in block.lb.y..=block.rb.y {
                    let curr_element = self.space[x as usize][y as usize][block.lb.z as usize - 1];
                    if curr_element != -1 && !seen.contains(&curr_element) {
                        self.graph[curr_element as usize].push(i);
                        self.reverse_graph[i].push(curr_element);
                        seen.insert(curr_element);
                    }
                }
            }
        }
    }
    fn find_maximum_chain(&mut self) -> u32 {
        let mut ret = 0;
        for i in 0..self.blocks.len() {
            let mut indegree = self
                .reverse_graph
                .iter()
                .map(|elem| elem.len())
                .collect::<Vec<_>>();
            let mut q = VecDeque::new();
            q.push_back(i);
            indegree[i] = 0;
            while let Some(elem) = q.pop_front() {
                ret += 1;
                for neighbor in self.graph[elem].iter() {
                    if indegree[*neighbor] > 0 {
                        indegree[*neighbor] -= 1;
                    }
                    if indegree[*neighbor] == 0 && *neighbor != i {
                        q.push_back(neighbor.to_owned());
                    }
                }
            }
        }
        ret - (self.blocks.len() as u32)
    }
}

impl Solution<u32, u32> for Day22 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut blocks = lines.map(Block::new).collect::<Vec<_>>();
        blocks.sort_by(|x, y| x.lb.z.cmp(&y.lb.z));
        let mut space_3d = Space3D::new(301);
        space_3d.simulate_fall(blocks);
        space_3d.build_graph();
        space_3d.count_disintegratable_block()
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
        let mut blocks = lines.map(Block::new).collect::<Vec<_>>();
        blocks.sort_by(|x, y| x.lb.z.cmp(&y.lb.z));
        let mut space_3d = Space3D::new(301);
        space_3d.simulate_fall(blocks);
        space_3d.build_graph();
        space_3d.find_maximum_chain()
    }
}
