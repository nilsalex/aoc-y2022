#![feature(test)]
#![feature(byte_slice_trim_ascii)]
extern crate test;

pub const INPUT: &[u8] = include_bytes!("input.txt");

const POWERS_OF_TEN: [u32; 10] = [
    1, 10, 100, 1000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000
];

fn u32_from_bytes(bytes: &[u8]) -> u32 {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| acc + (x - b'0') as u32 * POWERS_OF_TEN[ix])
}

fn part1(input: &[u8]) -> u32 {
    let mut max_cals_sum: u32 = 0;
    let mut cur_cals_sum: u32 = 0;

    for bytes in input.trim_ascii_end().split(|byte| *byte == b'\n') {
        if bytes.is_empty() {
            max_cals_sum = std::cmp::max(max_cals_sum, cur_cals_sum);
            cur_cals_sum = 0
        } else {
            cur_cals_sum += u32_from_bytes(bytes)
        }
    }

    max_cals_sum
}

fn part2(input: &[u8]) -> u32 {
    let mut cals = Vec::new();

    let mut cur_cals_sum: u32 = 0;

    for bytes in input.trim_ascii_end().split(|byte| *byte == b'\n') {
        if bytes.is_empty() {
            cals.push(cur_cals_sum);
            cur_cals_sum = 0
        } else {
            cur_cals_sum += u32_from_bytes(bytes)
        }
    }

    cals.select_nth_unstable_by(3, |a, b| b.cmp(a));
    cals.iter().take(3).sum()
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 69912)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 208180)
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
