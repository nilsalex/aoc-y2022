#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use bitvec::bitvec;
use bitvec::prelude::BitVec;
use std::collections::VecDeque;

const INPUT: &[u8] = include_bytes!("input.txt");

const GRID_SIZE: usize = 23;
const GRID_CENTRE_DOUBLED: usize = 23;

const POWERS_OF_TEN: [usize; 3] = [1, 10, 100];

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, byte)| {
            acc + (byte - b'0') as usize * POWERS_OF_TEN[index]
        })
}

struct Grid {
    grid: BitVec,
    cubes: Vec<(usize, usize, usize)>,
}

impl Grid {
    fn parse(bytes: &[u8]) -> Self {
        let mut cubes = Vec::new();
        let mut grid = bitvec![0; GRID_SIZE*GRID_SIZE*GRID_SIZE];

        bytes
            .trim_ascii_end()
            .split(|byte| *byte == b'\n')
            .map(|line| {
                let mut coords = line.split(|byte| *byte == b',');

                // Adding 2 to all coordinates to make sure we have a boundary of 1 and can work with
                // unsigned integers
                let x = usize_from_bytes(coords.next().unwrap()) + 2;
                let y = usize_from_bytes(coords.next().unwrap()) + 2;
                let z = usize_from_bytes(coords.next().unwrap()) + 2;
                (x, y, z)
            })
            .for_each(|(x, y, z)| {
                cubes.push((x, y, z));
                grid.set(z * GRID_SIZE * GRID_SIZE + y * GRID_SIZE + x, true);
            });

        Grid { grid, cubes }
    }

    fn get_cubes(&self) -> &[(usize, usize, usize)] {
        &self.cubes
    }

    fn get_index(position: &(usize, usize, usize)) -> usize {
        position.2 * GRID_SIZE * GRID_SIZE + position.1 * GRID_SIZE + position.0
    }

    fn is_cube(&self, position: &(usize, usize, usize)) -> bool {
        self.grid[Self::get_index(position)]
    }

    fn copy_neighbours_into_buffer(
        position: &(usize, usize, usize),
        neighbours: &mut [(usize, usize, usize)],
    ) {
        let (x, y, z) = (position.0, position.1, position.2);
        neighbours.copy_from_slice(&[
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]);
    }

    fn is_in_bounds(position: &(usize, usize, usize)) -> bool {
        GRID_CENTRE_DOUBLED.abs_diff(2 * position.0).pow(2)
            + GRID_CENTRE_DOUBLED.abs_diff(2 * position.1).pow(2)
            + GRID_CENTRE_DOUBLED.abs_diff(2 * position.2).pow(2)
            <= (GRID_CENTRE_DOUBLED - 1).pow(2)
    }
}

fn part1(input: &[u8]) -> usize {
    let grid = Grid::parse(input);
    let mut neighbours_buffer: Vec<(usize, usize, usize)> = vec![(0, 0, 0); 6];

    grid.get_cubes().iter().fold(0, |acc, cube| {
        Grid::copy_neighbours_into_buffer(cube, &mut neighbours_buffer);
        acc + neighbours_buffer
            .iter()
            .filter(|neighbour| !grid.is_cube(neighbour))
            .count()
    })
}

fn part2(input: &[u8]) -> usize {
    let grid = Grid::parse(input);
    let mut neighbours_buffer: Vec<(usize, usize, usize)> = vec![(0, 0, 0); 6];

    let mut visited: BitVec = bitvec![0; GRID_SIZE*GRID_SIZE*GRID_SIZE];

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((1, GRID_SIZE / 2, GRID_SIZE / 2));

    let mut surface_area: usize = 0;

    while let Some(position) = queue.pop_front() {
        let index = Grid::get_index(&position);
        if visited[index] {
            continue;
        }

        if grid.is_cube(&position) {
            surface_area += 1
        } else {
            visited.set(index, true);
            Grid::copy_neighbours_into_buffer(&position, &mut neighbours_buffer);
            for next in &neighbours_buffer {
                if !Grid::is_in_bounds(next) {
                    continue;
                }
                if visited[Grid::get_index(next)] {
                    continue;
                }
                queue.push_back(*next);
            }
        }
    }

    surface_area
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3448)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2052)
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| Grid::parse(INPUT))
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(INPUT))
    }
}
