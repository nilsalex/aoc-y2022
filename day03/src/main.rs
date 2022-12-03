#![feature(test)]
#![feature(byte_slice_trim_ascii)]
extern crate test;

use std::collections::HashSet;

pub const INPUT: &[u8] = include_bytes!("input.txt");
pub const NEWLINE: u8 = 10;

fn part1(input: &[u8]) -> usize {
    input
        .trim_ascii_end()
        .split(|byte| *byte == NEWLINE)
        .map(|bytes: &[u8]| {
            let num_bytes = bytes.len();

            let items1: HashSet<&u8> = HashSet::from_iter(bytes.iter().take(num_bytes / 2));
            let items2: HashSet<&u8> = HashSet::from_iter(bytes.iter().skip(num_bytes / 2));

            let common = **items1.intersection(&items2).next().unwrap();

            match common {
                0..=96 => (common - 38) as usize,
                _ => (common - 96) as usize,
            }
        })
        .sum()
}

fn part2(input: &[u8]) -> usize {
    let mut result = 0;

    let mut lines = input.trim_ascii_end().split(|byte| *byte == NEWLINE).peekable();

    while lines.peek().is_some() {
        let items1: HashSet<&u8> = HashSet::from_iter(lines.next().unwrap().iter());
        let items2: HashSet<&u8> = HashSet::from_iter(lines.next().unwrap().iter());
        let items3: HashSet<&u8> = HashSet::from_iter(lines.next().unwrap().iter());

        let inter1: HashSet<&u8> = HashSet::from_iter(items1.intersection(&items2).cloned());
        let common = inter1.intersection(&items3).next().unwrap();

        result += match common {
            0..=96 => (*common - 38) as usize,
            _ => (*common - 96) as usize,
        }
    };

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
