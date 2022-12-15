#![feature(test)]
extern crate test;

use std::cmp::max;

const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell { Air, Rock, Sand }

struct Grid {
    x_size: usize,
    y_size: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Cell {
        if x >= self.x_size {
            Cell::Air
        } else if y >= self.y_size {
            Cell::Air
        } else {
            self.cells[y * self.x_size + x]
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y*self.x_size + x] = cell
    }

    fn from_lines(lines: &Vec<Line>, add_bottom: bool) -> Self {
        let (x_max, y_max) = lines
            .iter()
            .fold((0, 0), |(x_max, y_max), line|
                (
                    max(max(x_max, line.start.0), line.end.0),
                    max(max(y_max, line.start.1), line.end.1),
                ),
            );
        let x_size = 2 * (x_max + 1) as usize;
        let y_size = if add_bottom {
            (y_max + 1) as usize
        } else {
            (y_max + 3) as usize
        };

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
                let (x, y) = ((line.start.0 + dx_ * i) as usize, (line.start.1 + dy_ * i) as usize);
                cells[y * x_size + x] = Cell::Rock;
            }
        }

        if add_bottom {
            for x in 0..x_size {
                cells[(y_size - 1) * x_size + x] = Cell::Rock
            }
        }

        Grid {x_size, y_size, cells}
    }
}

struct Line {
    start: (i16, i16),
    end: (i16, i16),
}

fn parse_node(input: &str) -> (i16, i16) {
    let mut numbers = input.split(',');
    (numbers.next().unwrap().parse().unwrap(), numbers.next().unwrap().parse().unwrap())
}

fn parse_lines(input: &str) -> Vec<Line> {
    let mut lines = Vec::new();

    for line in input.lines() {
        let mut nodes_it = line.split(" -> ");
        let mut current_node = nodes_it.next().unwrap();
        while let Some(next_node) = nodes_it.next() {
            lines.push(Line { start: parse_node(current_node), end: parse_node(next_node) });
            current_node = next_node
        }
    }

    lines
}

fn next(sx: usize, sy: usize, grid: &Grid) -> Option<(usize, usize)> {
    if grid.get(sx, sy+1) == Cell::Air {
        Some((sx, sy + 1))
    } else if grid.get(sx-1, sy+1) == Cell::Air {
        Some((sx - 1, sy + 1))
    } else if grid.get(sx+1, sy+1) == Cell::Air {
        Some((sx + 1, sy + 1))
    } else {
        None
    }
}

fn part1(input: &str) -> usize {
    let lines = parse_lines(input);
    let mut grid = Grid::from_lines(&lines, false);

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
    let mut grid = Grid::from_lines(&lines, false);

    let mut result = 0;

    loop {
        let (mut sx, mut sy) = (500, 0);

        while let Some((sx_, sy_)) = next(sx, sy, &grid) {
            (sx, sy) = (sx_, sy_);
        }

        grid.set(sx, sy, Cell::Sand);
        result += 1;

        if sy == 0 && sx == 500 {
            break;
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
        b.iter(|| Grid::from_lines(&lines, false))
    }

    #[bench]
    fn bench_grid_with_bottom(b: &mut Bencher) {
        let lines = parse_lines(INPUT);
        b.iter(|| Grid::from_lines(&lines, true))
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
