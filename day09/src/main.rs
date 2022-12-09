#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part1(input: &[u8]) -> usize {
    let (mut hx, mut hy): (i32, i32) = (0, 0);
    let (mut tx, mut ty): (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((tx, ty));

    for bytes in input.trim_ascii_end().split(|byte| *byte == b'\n') {
        let (dx, dy): (i32, i32) = match bytes[0] {
            b'U' => (-1, 0),
            b'D' => (1, 0),
            b'L' => (0, -1),
            b'R' => (0, 1),
            _ => panic!(),
        };
        let steps: u8 = std::str::from_utf8(&bytes[2..]).unwrap().parse().unwrap();

        for _ in 0..steps {
            hx += dx;
            hy += dy;
            match (hx-tx, hy-ty) {
                (-2, 0) => tx -= 1,
                (2, 0) => tx += 1,
                (0, -2) => ty -= 1,
                (0, 2) => ty += 1,
                (-2, 1) => {tx -= 1; ty += 1},
                (-2, -1) => {tx -= 1; ty -= 1},
                (2, 1) => {tx += 1; ty += 1},
                (2, -1) => {tx += 1; ty -= 1},
                (1, -2) => {tx += 1; ty -= 1},
                (-1, -2) => {tx -= 1; ty -= 1},
                (1, 2) => {tx += 1; ty += 1},
                (-1, 2) => {tx -= 1; ty += 1},
                (0, 0) => {},
                (0, 1) => {},
                (0, -1) => {},
                (1, 0) => {},
                (-1, 0) => {},
                (1, 1) => {},
                (1, -1) => {},
                (-1, 1) => {},
                (-1, -1) => {},
                _ => panic!(),
            }
            visited.insert((tx, ty));
        }
    }

    visited.len()
}

fn part2(input: &[u8]) -> usize {
    let mut rope: [(i32, i32); 10] = [(0,0); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(rope[9]);

    for bytes in input.trim_ascii_end().split(|byte| *byte == b'\n') {
        let (dx, dy): (i32, i32) = match bytes[0] {
            b'U' => (-1, 0),
            b'D' => (1, 0),
            b'L' => (0, -1),
            b'R' => (0, 1),
            _ => panic!(),
        };
        let steps: u8 = std::str::from_utf8(&bytes[2..]).unwrap().parse().unwrap();

        for _ in 0..steps {
            rope[0].0 += dx;
            rope[0].1 += dy;
            for segment in 1..10 {
                let (hx, hy) = rope[segment-1];
                let (mut tx, mut ty) = rope[segment];
                match (hx - tx, hy - ty) {
                    (-2, 0) => tx -= 1,
                    (2, 0) => tx += 1,
                    (0, -2) => ty -= 1,
                    (0, 2) => ty += 1,
                    (-2, 1) => {
                        tx -= 1;
                        ty += 1
                    },
                    (-2, -1) => {
                        tx -= 1;
                        ty -= 1
                    },
                    (2, 1) => {
                        tx += 1;
                        ty += 1
                    },
                    (2, -1) => {
                        tx += 1;
                        ty -= 1
                    },
                    (1, -2) => {
                        tx += 1;
                        ty -= 1
                    },
                    (-1, -2) => {
                        tx -= 1;
                        ty -= 1
                    },
                    (1, 2) => {
                        tx += 1;
                        ty += 1
                    },
                    (-1, 2) => {
                        tx -= 1;
                        ty += 1
                    },
                    (0, 0) => {},
                    (0, 1) => {},
                    (0, -1) => {},
                    (1, 0) => {},
                    (-1, 0) => {},
                    (1, 1) => {},
                    (1, -1) => {},
                    (-1, 1) => {},
                    (-1, -1) => {},
                    (-2, -2) => {tx -= 1; ty -= 1},
                    (-2, 2) => {tx -= 1; ty += 1},
                    (2, -2) => {tx += 1; ty -= 1},
                    (2, 2) => {tx += 1; ty += 1},
                    _ => panic!(),
                }
                rope[segment] = (tx, ty)
            }
            visited.insert(rope[9]);
        }
    }

    visited.len()
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
        assert_eq!(part1(INPUT), 6311)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2482)
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
