#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(input: &[u8], start: u8, end: u8, direction: Direction) -> usize {
    let grid: Vec<&[u8]> = input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .collect();

    let (x_size, y_size) = (grid[0].len(), grid.len());
    let (mut x0, mut y0) = (0, 0);

    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == start {
                (x0, y0) = (x, y);
                break 'outer;
            }
        }
    }

    let mut visited: Vec<bool> = vec![false; x_size * y_size];
    let mut queue: VecDeque<State> = VecDeque::new();
    visited[y0 * x_size + x0];
    queue.push_back(State { cost: 0, position: (x0, y0) });

    while let Some(State { cost, position }) = queue.pop_front() {
        if grid[position.1][position.0] == end {
            return cost;
        }
        if visited[position.1 * x_size + position.0] {
            continue;
        }
        for (next_x, next_y) in next_cells(position.0, position.1, x_size, y_size, direction, &grid)
        {
            let next = State {
                cost: cost + 1,
                position: (next_x, next_y),
            };
            queue.push_back(next);
            visited[]
            dist.insert(next.position, next.cost);
        }
    }

    panic!()
}

fn next_cells(
    x: usize,
    y: usize,
    x_size: usize,
    y_size: usize,
    direction: Direction,
    grid: &[&[u8]],
) -> Vec<(usize, usize)> {
    let (x_, y_, x_size_, y_size_) = (x as isize, y as isize, x_size as isize, y_size as isize);

    let current_elevation = match grid[y][x] {
        b'S' => b'a',
        b'E' => b'z',
        h => h,
    };

    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .iter()
        .map(|(dx, dy)| (x_ + dx, y_ + dy))
        .filter(|(x_new, y_new)| *x_new >= 0 && *y_new >= 0 && *x_new < x_size_ && *y_new < y_size_)
        .map(|(x_new, y_new)| (x_new as usize, y_new as usize))
        .filter(|(x_new, y_new)| {
            let new_elevation = match grid[*y_new][*x_new] {
                b'S' => b'a',
                b'E' => b'z',
                h => h,
            };
            match direction {
                Direction::Up => current_elevation + 1 >= new_elevation,
                Direction::Down => new_elevation + 1 >= current_elevation,
            }
        })
        .collect::<Vec<(usize, usize)>>()
}

fn part1(input: &[u8]) -> usize {
    solve(input, b'S', b'E', Direction::Up)
}

fn part2(input: &[u8]) -> usize {
    solve(input, b'E', b'a', Direction::Down)
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
