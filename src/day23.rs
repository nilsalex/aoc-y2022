use std::collections::{HashMap, HashSet};

pub(crate) const INPUT: &[u8] = include_bytes!("../inputs/day23.txt");
// const INPUT: &[u8] = include_bytes!("input_test.txt");

const DIRS: [Dir; 4] = [Dir::U, Dir::D, Dir::L, Dir::R];

#[derive(Debug, Copy, Clone)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn next_position(&self, position: &(isize, isize)) -> (isize, isize) {
        match self {
            Dir::U => (position.0 - 1, position.1),
            Dir::D => (position.0 + 1, position.1),
            Dir::L => (position.0, position.1 - 1),
            Dir::R => (position.0, position.1 + 1),
        }
    }

    fn test(&self, position: &(isize, isize), positions: &HashSet<(isize, isize)>) -> bool {
        let (row, col) = *position;
        match self {
            Dir::U => {
                !positions.contains(&(row - 1, col))
                    && !positions.contains(&(row - 1, col - 1))
                    && !positions.contains(&(row - 1, col + 1))
            }
            Dir::D => {
                !positions.contains(&(row + 1, col))
                    && !positions.contains(&(row + 1, col - 1))
                    && !positions.contains(&(row + 1, col + 1))
            }
            Dir::L => {
                !positions.contains(&(row, col - 1))
                    && !positions.contains(&(row - 1, col - 1))
                    && !positions.contains(&(row + 1, col - 1))
            }
            Dir::R => {
                !positions.contains(&(row, col + 1))
                    && !positions.contains(&(row - 1, col + 1))
                    && !positions.contains(&(row + 1, col + 1))
            }
        }
    }
}

fn get_bounds<'a>(
    positions: impl Iterator<Item = &'a (isize, isize)>,
) -> (isize, isize, isize, isize) {
    positions.fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_row, max_row, min_col, max_col), (row, col)| {
            (
                min_row.min(*row),
                max_row.max(*row),
                min_col.min(*col),
                max_col.max(*col),
            )
        },
    )
}

fn evolve(positions: &HashSet<(isize, isize)>, step: usize) -> (HashSet<(isize, isize)>, bool) {
    let mut target_map: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    let mut target_counts: HashMap<(isize, isize), usize> = HashMap::new();

    let dirs = [
        DIRS[step % 4],
        DIRS[(step + 1) % 4],
        DIRS[(step + 2) % 4],
        DIRS[(step + 3) % 4],
    ];

    for position in positions.iter() {
        let free_dirs: Vec<Dir> = dirs
            .iter()
            .cloned()
            .filter(|dir| dir.test(position, positions))
            .collect();

        if free_dirs.len() == 4 {
            continue;
        }

        if let Some(dir) = free_dirs.get(0) {
            let next_position = dir.next_position(position);
            target_map.insert(*position, next_position);
            if let Some(count) = target_counts.get_mut(&next_position) {
                *count += 1;
            } else {
                target_counts.insert(next_position, 1);
            }
        }
    }

    let mut next_positions = HashSet::new();

    let mut finished = true;

    for position in positions.iter() {
        if let Some(target) = target_map.get(position) {
            match target_counts.get(target) {
                None => {
                    panic!();
                }
                Some(0) => {
                    panic!();
                }
                Some(1) => {
                    next_positions.insert(*target);
                    finished = false;
                }
                _ => {
                    next_positions.insert(*position);
                }
            }
        } else {
            next_positions.insert(*position);
        }
    }

    (next_positions, finished)
}

fn parse_positions(input: &[u8]) -> HashSet<(isize, isize)> {
    input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, byte)| ((row as isize, col as isize), *byte))
                .collect::<Vec<((isize, isize), u8)>>()
        })
        .filter(|(_, byte)| *byte == b'#')
        .map(|(a, _)| a)
        .collect()
}

pub(crate) fn part1(input: &[u8]) -> isize {
    let mut positions = parse_positions(input);

    for i in 0..10 {
        let (next_positions, finished) = evolve(&positions, i);
        if finished {
            break;
        }
        positions = next_positions
    }

    let (row_min, row_max, col_min, col_max) = get_bounds(positions.iter());

    (row_max - row_min + 1) * (col_max - col_min + 1) - positions.len() as isize
}

pub(crate) fn part2(input: &[u8]) -> usize {
    let mut positions = parse_positions(input);
    let mut round = 0;

    loop {
        let (next_positions, finished) = evolve(&positions, round);
        if finished {
            break;
        }
        positions = next_positions;
        round += 1;
    }

    round + 1
}
