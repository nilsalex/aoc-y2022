#![feature(test)]
#![feature(byte_slice_trim_ascii)]

pub const INPUT: &[u8] = include_bytes!("../../inputs/day02.txt");

fn part1(input: &[u8]) -> u32 {
    input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .map(|bytes| {
            let a = bytes[0] - b'A';
            let b = bytes[2] - b'X';
            (1 + b + ((4 + b - a) % 3) * 3) as u32 // add 4 instead of 1 because 4 + b - a is always > 0
        })
        .sum()
}

fn part2(input: &[u8]) -> u32 {
    input
        .split_inclusive(|byte| *byte == b'\n')
        .map(|bytes| {
            let a = bytes[0] - b'A';
            let b = bytes[2] - b'X';
            (1 + 3 * b + (a + b + 2) % 3) as u32
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
        assert_eq!(part1(INPUT), 15572)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 16098)
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
