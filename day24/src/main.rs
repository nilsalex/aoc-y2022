#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

const INPUT: &[u8] = include_bytes!("input.txt");
// const INPUT: &[u8] = include_bytes!("input_test.txt");

#[derive(Clone)]
struct Grid {
    arrows_up: Vec<Vec<isize>>,
    arrows_down: Vec<Vec<isize>>,
    arrows_left: Vec<Vec<isize>>,
    arrows_right: Vec<Vec<isize>>,
    num_rows: isize,
    num_cols: isize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn dist(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.row.cmp(&other.row).then(self.col.cmp(&other.col))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Grid {
    fn parse(input: &[u8]) -> Self {
        let num_cols = input.iter().position(|byte| *byte == b'\n').unwrap() - 2;
        let num_rows = input
            .trim_ascii_end()
            .iter()
            .filter(|byte| **byte == b'\n')
            .count()
            - 1;

        let mut arrows_up: Vec<Vec<isize>> = vec![Vec::new(); num_cols];
        let mut arrows_down: Vec<Vec<isize>> = vec![Vec::new(); num_cols];
        let mut arrows_left: Vec<Vec<isize>> = vec![Vec::new(); num_rows];
        let mut arrows_right: Vec<Vec<isize>> = vec![Vec::new(); num_rows];

        input
            .trim_ascii_end()
            .split(|byte| *byte == b'\n')
            .skip(1)
            .take(num_rows)
            .enumerate()
            .for_each(|(row, line)| {
                line.iter()
                    .skip(1)
                    .take(num_cols)
                    .enumerate()
                    .for_each(|(col, byte)| match byte {
                        b'^' => arrows_up[col].push(row as isize),
                        b'v' => arrows_down[col].push(row as isize),
                        b'<' => arrows_left[row].push(col as isize),
                        b'>' => arrows_right[row].push(col as isize),
                        b'.' => {}
                        _ => panic!(),
                    })
            });

        let num_rows = num_rows as isize;
        let num_cols = num_cols as isize;

        Self {
            arrows_up,
            arrows_down,
            arrows_left,
            arrows_right,
            num_rows,
            num_cols,
        }
    }

    fn in_bounds(&self, pos: &Position) -> bool {
        pos.col >= 0 && pos.row >= 0 && pos.col < self.num_cols && pos.row < self.num_rows
            || pos.row == -1 && pos.col == 0
            || pos.row == self.num_rows && pos.col == self.num_cols - 1
    }

    fn is_allowed(&self, pos: &Position, step: usize) -> bool {
        if pos.row == -1 && pos.col == 0 {
            return true;
        }

        if pos.row == self.num_rows && pos.col == self.num_cols - 1 {
            return true;
        }

        for arrow in &self.arrows_up[pos.col as usize] {
            if (arrow - step as isize).rem_euclid(self.num_rows) == pos.row {
                return false;
            }
        }

        for arrow in &self.arrows_down[pos.col as usize] {
            if (arrow + step as isize).rem_euclid(self.num_rows) == pos.row {
                return false;
            }
        }

        for arrow in &self.arrows_left[pos.row as usize] {
            if (arrow - step as isize).rem_euclid(self.num_cols) == pos.col {
                return false;
            }
        }

        for arrow in &self.arrows_right[pos.row as usize] {
            if (arrow + step as isize).rem_euclid(self.num_cols) == pos.col {
                return false;
            }
        }

        true
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct State {
    depth: usize,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.depth
            .cmp(&other.depth)
            .then(self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(grid: &Grid, initial_state: &State, target_pos: &Position) -> State {
    let mut queue: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();
    let mut visited: HashSet<State> = HashSet::new();

    queue.push(Reverse((
        initial_state.position.dist(target_pos),
        *initial_state,
    )));
    visited.insert(*initial_state);

    loop {
        if let Some(Reverse((_, state))) = queue.pop() {
            if &state.position == target_pos {
                break state;
            }

            let next_states: Vec<State> = [
                State {
                    position: Position {
                        row: state.position.row - 1,
                        col: state.position.col,
                    },
                    depth: state.depth + 1,
                },
                State {
                    position: Position {
                        row: state.position.row + 1,
                        col: state.position.col,
                    },
                    depth: state.depth + 1,
                },
                State {
                    position: Position {
                        row: state.position.row,
                        col: state.position.col - 1,
                    },
                    depth: state.depth + 1,
                },
                State {
                    position: Position {
                        row: state.position.row,
                        col: state.position.col + 1,
                    },
                    depth: state.depth + 1,
                },
                State {
                    position: state.position,
                    depth: state.depth + 1,
                },
            ]
            .into_iter()
            .filter(|state| {
                grid.in_bounds(&state.position) && grid.is_allowed(&state.position, state.depth)
            })
            .collect();

            for next_state in next_states {
                if !visited.contains(&next_state) {
                    let heuristic = next_state.depth + next_state.position.dist(target_pos);
                    visited.insert(next_state);
                    queue.push(Reverse((heuristic, next_state)));
                }
            }
        } else {
            panic!()
        }
    }
}

fn part1(input: &[u8]) -> usize {
    let grid = Grid::parse(input);

    let begin_pos = Position { row: -1, col: 0 };
    let end_pos = Position {
        row: grid.num_rows,
        col: grid.num_cols - 1,
    };

    let initial_state = State {
        depth: 0,
        position: begin_pos,
    };

    let final_state = bfs(&grid, &initial_state, &end_pos);

    final_state.depth
}

fn part2(input: &[u8]) -> usize {
    let grid = Grid::parse(input);

    let begin_pos = Position { row: -1, col: 0 };
    let end_pos = Position {
        row: grid.num_rows,
        col: grid.num_cols - 1,
    };

    let initial_state = State {
        depth: 0,
        position: begin_pos,
    };

    let there = bfs(&grid, &initial_state, &end_pos);
    let and_back_again = bfs(&grid, &there, &begin_pos);
    let and_there_again = bfs(&grid, &and_back_again, &end_pos);

    and_there_again.depth
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
        assert_eq!(part1(INPUT), 308)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 908)
    }

    #[bench]
    fn bench_parse_grid(b: &mut Bencher) {
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
