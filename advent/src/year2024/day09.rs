use crate::solver::AdventSolver;
use itertools::Itertools;

pub struct Advent2024Day09Solver {
    disk_map: Vec<u8>,
}

impl Advent2024Day09Solver {
    pub fn new(input: &str) -> Self {
        Self {
            disk_map: input.chars().map(|c| c as u8 - b'0').collect(),
        }
    }
}

impl AdventSolver for Advent2024Day09Solver {
    fn solve_part1(&self) -> usize {
        let mut fs = FileSystem::from_disk_map(&self.disk_map);
        fs.block_compact();
        fs.check_sum()
    }

    fn solve_part2(&self) -> usize {
        // todo slow (13s)
        let mut fs = FileSystem::from_disk_map(&self.disk_map);
        fs.file_compact();
        fs.check_sum()
    }
}

struct FileSystem {
    blocks: Vec<Option<usize>>,
    files: Vec<(usize, usize)>,
}

impl FileSystem {
    fn from_disk_map(disk_map: &[u8]) -> Self {
        let mut blocks = Vec::new();
        let mut files = Vec::new();
        for i in 0..disk_map.len() {
            if i % 2 == 1 {
                blocks.extend(vec![None; disk_map[i] as usize]);
            } else {
                let id = i / 2;
                blocks.extend(vec![Some(id); disk_map[i] as usize]);
                files.push((id, disk_map[i] as usize));
            }
        }
        Self { blocks, files }
    }

    fn block_compact(&mut self) {
        let mut i = 0;
        let mut j = self.blocks.len() - 1;
        while i < j {
            if self.blocks[i].is_some() {
                i += 1;
                continue;
            }
            if self.blocks[j].is_none() {
                j -= 1;
                continue;
            }
            self.blocks.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    fn file_compact(&mut self) {
        for &(id, size) in self.files.iter().rev() {
            if let Some((new_position, _)) = self
                .blocks
                .windows(size)
                .enumerate()
                .find(|(_np, w)| w.iter().all(Option::is_none))
            {
                let old_position = self
                    .blocks
                    .iter()
                    .find_position(|b| b.is_some_and(|b| b == id))
                    .unwrap()
                    .0;
                if new_position > old_position {
                    continue;
                }
                for i in 0..size {
                    self.blocks.swap(new_position + i, old_position + i);
                }
            }
        }
    }

    fn check_sum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.map(|b| i * b))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn finds_checksum_of_block_compacted_system() {
        let solver = Advent2024Day09Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 1928);
    }

    #[test]
    fn finds_checksum_of_file_compacted_system() {
        let solver = Advent2024Day09Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 2858);
    }
}
