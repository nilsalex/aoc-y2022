#![feature(test)]
#![feature(byte_slice_trim_ascii)]
extern crate test;

use itertools::Itertools;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day03.txt");
pub const NEWLINE: u8 = 10;

fn part1(input: &[u8]) -> usize {
    input
        .trim_ascii_end()
        .split(|byte| *byte == NEWLINE)
        .map(|bytes: &[u8]| {
            let num_bytes = bytes.len();

            for item1 in bytes.iter().take(num_bytes / 2) {
                for item2 in bytes.iter().skip(num_bytes / 2) {
                    if *item1 == *item2 {
                        return *item1;
                    }
                }
            }
            panic!()
        })
        .map(|common| match common {
            0..=96 => (common - 38) as usize,
            _ => (common - 96) as usize,
        })
        .sum()
}

fn part2(input: &[u8]) -> usize {
    let lines = input.trim_ascii_end().split(|byte| *byte == NEWLINE);

    lines
        .tuples()
        .map(|(line1, line2, line3)| {
            for item1 in line1 {
                for item2 in line2 {
                    for item3 in line3 {
                        if *item1 == *item2 && *item2 == *item3 {
                            return *item1;
                        }
                    }
                }
            }
            panic!()
        })
        .map(|common| match common {
            0..=96 => (common - 38) as usize,
            _ => (common - 96) as usize,
        })
        .sum()
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
        assert_eq!(part1(INPUT), 7691)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2508)
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
