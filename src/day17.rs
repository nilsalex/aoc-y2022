extern crate test;

use std::collections::HashSet;

pub(crate) const INPUT: &[u8] = include_bytes!("../inputs/day17.txt");

#[derive(Copy, Clone, Debug)]
enum Instruction {
    L,
    R,
}

#[derive(Debug)]
struct Instructions {
    instructions: Vec<u8>,
}

impl Instructions {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            instructions: Vec::from(bytes.trim_ascii_end()),
        }
    }

    fn get(&self, pos: usize) -> Instruction {
        match self.instructions[pos % self.instructions.len()] {
            b'<' => Instruction::L,
            b'>' => Instruction::R,
            _ => panic!(),
        }
    }
}

enum FallResult {
    Resting,
    Moving,
}

enum Piece {
    Plus,
    Minus,
    L,
    I,
    Square,
}

impl Piece {
    const fn next(&self) -> Self {
        match self {
            Piece::Plus => Piece::Minus,
            Piece::Minus => Piece::L,
            Piece::L => Piece::I,
            Piece::I => Piece::Square,
            Piece::Square => Piece::Plus,
        }
    }

    fn pattern(&self) -> Vec<(usize, usize)> {
        match self {
            Piece::Plus => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Piece::Minus => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            Piece::L => vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            Piece::I => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Piece::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }

    fn initial_pattern(&self, height: usize) -> Vec<(usize, usize)> {
        let mut pattern = self.pattern();
        for (y, x) in pattern.iter_mut() {
            *y += height + 3;
            *x += 2;
        }
        pattern
    }
}

fn shift(
    piece_pattern: &mut Vec<(usize, usize)>,
    rocks: &HashSet<(usize, usize)>,
    instruction: Instruction,
) {
    let mut next_pattern = piece_pattern.clone();
    match instruction {
        Instruction::L => {
            for (y, x) in next_pattern.iter_mut() {
                if *x == 0 {
                    return;
                }
                *x -= 1;
                if rocks.contains(&(*y, *x)) {
                    return;
                }
            }
        }
        Instruction::R => {
            for (y, x) in next_pattern.iter_mut() {
                if *x >= 6 {
                    return;
                }
                *x += 1;
                if rocks.contains(&(*y, *x)) {
                    return;
                }
            }
        }
    }
    std::mem::swap(&mut next_pattern, piece_pattern)
}

fn fall(
    piece_pattern: &mut Vec<(usize, usize)>,
    rocks: &mut HashSet<(usize, usize)>,
) -> FallResult {
    let mut next_pattern = piece_pattern.clone();
    for (y, x) in next_pattern.iter_mut() {
        if *y == 0 {
            rocks.extend(piece_pattern.iter());
            return FallResult::Resting;
        }
        *y -= 1;
        if rocks.contains(&(*y, *x)) {
            rocks.extend(piece_pattern.iter());
            return FallResult::Resting;
        }
    }
    std::mem::swap(&mut next_pattern, piece_pattern);

    FallResult::Moving
}

pub(crate) fn part1(input: &[u8]) -> usize {
    let instructions = Instructions::from_bytes(input);

    let mut rocks: HashSet<(usize, usize)> = HashSet::new();
    let mut height: usize = 0;
    let mut piece = Piece::Plus;
    let mut piece_pattern = piece.initial_pattern(height);

    let mut cycle: usize = 0;
    let mut pieces_count: usize = 0;

    while pieces_count < 2022 {
        let instruction = instructions.get(cycle);
        shift(&mut piece_pattern, &rocks, instruction);
        match fall(&mut piece_pattern, &mut rocks) {
            FallResult::Moving => {}
            FallResult::Resting => {
                height = std::cmp::max(
                    height,
                    piece_pattern.iter().map(|(y, _)| *y + 1).max().unwrap(),
                );
                piece = piece.next();
                piece_pattern = piece.initial_pattern(height);
                pieces_count += 1;
            }
        }

        cycle += 1;
    }

    height
}

pub(crate) fn part2(input: &[u8]) -> usize {
    let instructions = Instructions::from_bytes(input);

    let mut rocks: HashSet<(usize, usize)> = HashSet::new();
    let mut height: usize = 0;
    let mut piece = Piece::Plus;
    let mut piece_pattern = piece.initial_pattern(height);

    let mut cycle: usize = 0;
    let mut pieces_count: usize = 0;

    let mut diffs: Vec<usize> = Vec::new();

    while pieces_count < 200000 {
        let instruction = instructions.get(cycle);
        shift(&mut piece_pattern, &rocks, instruction);
        match fall(&mut piece_pattern, &mut rocks) {
            FallResult::Moving => {}
            FallResult::Resting => {
                let old_height = height;
                height = std::cmp::max(
                    height,
                    piece_pattern.iter().map(|(y, _)| *y + 1).max().unwrap(),
                );
                piece = piece.next();
                piece_pattern = piece.initial_pattern(height);
                pieces_count += 1;

                diffs.push(height - old_height);
            }
        }

        cycle += 1;
    }

    let before: usize;
    let mut window_size = if instructions.instructions.len() % 5 == 0 {
        instructions.instructions.len()
    } else {
        instructions.instructions.len() * 5
    };

    'outer: loop {
        for i in 0..diffs.len() {
            if i + 2 * window_size >= diffs.len() {
                break;
            }
            let window_1 = &diffs[i..i + window_size];
            let window_2 = &diffs[i + window_size..i + 2 * window_size];
            if window_1 == window_2 {
                before = i;
                break 'outer;
            }
        }
        window_size += 5;
    }

    let total = 1000000000000;
    let cycle_length = window_size;
    let fitting_cycles = (total - before) / cycle_length;
    let after = (total - before) % cycle_length;

    let sum_before: usize = diffs.iter().take(before).sum();
    let sum_fitting: usize =
        diffs.iter().skip(before).take(cycle_length).sum::<usize>() * fitting_cycles;
    let sum_after: usize = diffs.iter().skip(before).take(after).sum();

    sum_before + sum_fitting + sum_after
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3119)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1536994219669)
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
