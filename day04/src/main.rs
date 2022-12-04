#![feature(test)]
#![feature(byte_slice_trim_ascii)]
extern crate test;

const INPUT: &[u8] = include_bytes!("input.txt");

const POWERS_OF_TEN: [u8; 3] = [1, 10, 100];

fn u8_from_bytes(bytes: &[u8]) -> u8 {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| acc + (x - b'0') * POWERS_OF_TEN[ix])
}

fn numbers_from_line(bytes: &[u8]) -> (u8, u8, u8, u8) {
    let mut numbers = bytes.split(|byte| *byte == b',' || *byte == b'-');

    (
        u8_from_bytes(numbers.next().unwrap()),
        u8_from_bytes(numbers.next().unwrap()),
        u8_from_bytes(numbers.next().unwrap()),
        u8_from_bytes(numbers.next().unwrap()),
    )
}

fn part1(input: &[u8]) -> usize {
    input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .map(numbers_from_line)
        .filter(|(a, b, c, d)| a >= c && b <= d || c >= a && d <= b)
        .count()
}

fn part2(input: &[u8]) -> usize {
    input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .map(numbers_from_line)
        .filter(|(a, b, c, d)| !(a < c && b < c || c < a && d < a))
        .count()
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
        assert_eq!(part1(INPUT), 571)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 917)
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
