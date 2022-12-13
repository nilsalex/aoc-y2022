#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &[u8]) -> usize {
    let grid: Vec<&[u8]> = input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .collect();

    let (x_size, y_size) = (grid[0].len(), grid.len());
    let (mut x0, mut y0) = (0, 0);

    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'S' {
                (x0, y0) = (x, y);
                break 'outer;
            }
        }
    }

    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    dist.insert((x0, y0), 0);
    heap.push(State { cost: 0, position: (x0, y0) });

    while let Some(State { cost, position }) = heap.pop() {
        if grid[position.1][position.0] == b'E' { return cost; }
        if &cost > dist.get(&position).unwrap_or(&usize::MAX) { continue; }
        for (next_x, next_y) in next_cells(position.0, position.1, x_size, y_size, &grid) {
            let next = State { cost: cost + 1, position: (next_x, next_y) };
            if next.cost < *dist.get(&next.position).unwrap_or(&usize::MAX) {
                heap.push(next);
                dist.insert(next.position, next.cost);
            }
        }
    }

    panic!()
}

fn next_cells(x: usize, y: usize, x_size: usize, y_size: usize, grid: &[&[u8]]) -> Vec<(usize, usize)> {
    let (x_, y_, x_size_, y_size_) = (x as isize, y as isize, x_size as isize, y_size as isize);
    let mut current_elevation = grid[y][x];
    if current_elevation == b'S' {
        current_elevation = b'a'
    }

    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .iter()
        .map(|(dx, dy)| (x_ + dx, y_ + dy))
        .filter(|(x_new, y_new)| *x_new >= 0 && *y_new >= 0 && *x_new < x_size_ && *y_new < y_size_)
        .map(|(x_new, y_new)| (x_new as usize, y_new as usize))
        .filter(|(x_new, y_new)| {
            let mut new_elevation = grid[*y_new][*x_new];
            if new_elevation == b'E' {
                new_elevation = b'z'
            }
            current_elevation + 1 >= new_elevation
        })
        .collect::<Vec<(usize, usize)>>()
}

fn part2(input: &[u8]) -> usize {
    let grid: Vec<&[u8]> = input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .collect();

    let (x_size, y_size) = (grid[0].len(), grid.len());

    let mut result = usize::MAX;

    for (y, row) in grid.iter().enumerate() {
        'inner: for (x, cell) in row.iter().enumerate() {
            if *cell == b'S' || *cell == b'a' {
                let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
                let mut heap: BinaryHeap<State> = BinaryHeap::new();
                dist.insert((x, y), 0);
                heap.push(State { cost: 0, position: (x, y) });

                while let Some(State { cost, position }) = heap.pop() {
                    if grid[position.1][position.0] == b'E' {
                        result = std::cmp::min(result, cost);
                        continue 'inner;
                    }
                    if &cost > dist.get(&position).unwrap_or(&usize::MAX) { continue; }
                    for (next_x, next_y) in next_cells(position.0, position.1, x_size, y_size, &grid) {
                        let next = State { cost: cost + 1, position: (next_x, next_y) };
                        if next.cost < *dist.get(&next.position).unwrap_or(&usize::MAX) {
                            heap.push(next);
                            dist.insert(next.position, next.cost);
                        }
                    }
                }
            }
        }
    }

    result
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
        assert_eq!(part1(INPUT), 394)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 388)
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
