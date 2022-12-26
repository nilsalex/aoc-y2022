#![feature(test)]
extern crate test;

use std::cmp::max;
use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

struct Grid {
    x_size: usize,
    y_size: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Cell {
        if x >= self.x_size || y >= self.y_size {
            Cell::Air
        } else {
            self.cells[y * self.x_size + x]
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y * self.x_size + x] = cell
    }

    fn from_lines(lines: &Vec<Line>) -> Self {
        let (x_max, y_max) = lines.iter().fold((0, 0), |(x_max, y_max), line| {
            (
                max(max(x_max, line.start.0), line.end.0),
                max(max(y_max, line.start.1), line.end.1),
            )
        });
        let x_size = 2 * (x_max + 1) as usize;
        let y_size = (y_max + 1) as usize;

        let mut cells = vec![Cell::Air; x_size * y_size];

        for line in lines {
            let (dx, dy) = (line.end.0 - line.start.0, line.end.1 - line.start.1);

            let steps = max(dx.abs(), dy.abs());
            let (dx_, dy_) = if dx == 0 && dy == 0 {
                (0, 0)
            } else {
                (dx / steps, dy / steps)
            };

            for i in 0..=steps {
                let (x, y) = (
                    (line.start.0 + dx_ * i) as usize,
                    (line.start.1 + dy_ * i) as usize,
                );
                cells[y * x_size + x] = Cell::Rock;
            }
        }

        Grid {
            x_size,
            y_size,
            cells,
        }
    }
}

struct Line {
    start: (i16, i16),
    end: (i16, i16),
}

fn parse_node(input: &str) -> (i16, i16) {
    let mut numbers = input.split(',');
    (
        numbers.next().unwrap().parse().unwrap(),
        numbers.next().unwrap().parse().unwrap(),
    )
}

fn parse_lines(input: &str) -> Vec<Line> {
    let mut lines = Vec::new();

    for line in input.lines() {
        let mut nodes_it = line.split(" -> ");
        let mut current_node = nodes_it.next().unwrap();
        for next_node in nodes_it {
            lines.push(Line {
                start: parse_node(current_node),
                end: parse_node(next_node),
            });
            current_node = next_node
        }
    }

    lines
}

fn next_all(sx: usize, sy: usize, grid: &Grid) -> Vec<(usize, usize)> {
    [(sx, sy + 1), (sx - 1, sy + 1), (sx + 1, sy + 1)]
        .into_iter()
        .filter(|(x, y)| grid.get(*x, *y) == Cell::Air)
        .collect()
}

fn next(sx: usize, sy: usize, grid: &Grid) -> Option<(usize, usize)> {
    if grid.get(sx, sy + 1) == Cell::Air {
        Some((sx, sy + 1))
    } else if grid.get(sx - 1, sy + 1) == Cell::Air {
        Some((sx - 1, sy + 1))
    } else if grid.get(sx + 1, sy + 1) == Cell::Air {
        Some((sx + 1, sy + 1))
    } else {
        None
    }
}

fn part1(input: &str) -> usize {
    let lines = parse_lines(input);
    let mut grid = Grid::from_lines(&lines);

    let mut result = 0;

    'outer: loop {
        let (mut sx, mut sy) = (500, 0);

        while let Some((sx_, sy_)) = next(sx, sy, &grid) {
            if sy >= grid.y_size {
                break 'outer;
            }
            (sx, sy) = (sx_, sy_);
        }

        grid.set(sx, sy, Cell::Sand);
        result += 1;
    }

    result
}

fn part2(input: &str) -> usize {
    let lines = parse_lines(input);
    let grid = Grid::from_lines(&lines);

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut result = 1;

    queue.push_back((500, 0));
    visited.insert((500, 0));

    while let Some((x, y)) = queue.pop_front() {
        for (next_x, next_y) in next_all(x, y, &grid) {
            if y + 1 > grid.y_size {
                continue;
            }
            if visited.contains(&(next_x, next_y)) {
                continue;
            }
            queue.push_back((next_x, next_y));
            visited.insert((next_x, next_y));
            result += 1;
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
        assert_eq!(part1(INPUT), 757)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 24943)
    }

    #[bench]
    fn bench_parser(b: &mut Bencher) {
        b.iter(|| parse_lines(INPUT))
    }

    #[bench]
    fn bench_grid(b: &mut Bencher) {
        let lines = parse_lines(INPUT);
        b.iter(|| Grid::from_lines(&lines))
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
