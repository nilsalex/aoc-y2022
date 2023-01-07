#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../../inputs/day09.txt");

const POWERS_OF_TEN: [u8; 3] = [1, 10, 100];

fn u8_from_bytes(bytes: &[u8]) -> u8 {
    bytes
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (ix, x)| acc + (x - b'0') * POWERS_OF_TEN[ix])
}

fn part1(input: &[u8]) -> usize {
    let mut head: (i16, i16) = (0, 0);
    let mut tail: (i16, i16) = (0, 0);
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    visited.insert(tail);

    for bytes in input.trim_ascii_end().split(|byte| *byte == b'\n') {
        let (dx, dy): (i16, i16) = match bytes[0] {
            b'U' => (-1, 0),
            b'D' => (1, 0),
            b'L' => (0, -1),
            b'R' => (0, 1),
            _ => panic!(),
        };
        let steps = u8_from_bytes(&bytes[2..]);

        for _ in 0..steps {
            head.0 += dx;
            head.1 += dy;
            let (diff_x, diff_y) = (head.0 - tail.0, head.1 - tail.1);
            if diff_x.abs() > 1 || diff_y.abs() > 1 {
                tail.0 += diff_x.signum();
                tail.1 += diff_y.signum();
                visited.insert(tail);
            }
        }
    }

    visited.len()
}

fn part2(input: &[u8]) -> usize {
    let mut rope: [i32; 20] = [0; 20];
    let mut visited: HashSet<u64> = HashSet::new();
    visited.insert((((rope[18] as u32) as u64) << 32) | ((rope[19] as u32) as u64));

    for bytes in input.trim_ascii_end().split(|byte| *byte == b'\n') {
        let (dx, dy): (i32, i32) = match bytes[0] {
            b'U' => (-1, 0),
            b'D' => (1, 0),
            b'L' => (0, -1),
            b'R' => (0, 1),
            _ => panic!(),
        };
        let steps = u8_from_bytes(&bytes[2..]);

        for _ in 0..steps {
            rope[0] += dx;
            rope[1] += dy;
            for segment in 0..9 {
                let (diff_x, diff_y): (i32, i32) = (
                    rope[2 * segment] - rope[2 * segment + 2],
                    rope[2 * segment + 1] - rope[2 * segment + 3],
                );
                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    rope[2 * segment + 2] += diff_x.signum();
                    rope[2 * segment + 3] += diff_y.signum();
                }
            }
            visited.insert((((rope[18] as u32) as u64) << 32) | ((rope[19] as u32) as u64));
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
