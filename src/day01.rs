extern crate test;

pub const INPUT: &[u8] = include_bytes!("../inputs/day01.txt");

const POWERS_OF_TEN: [u32; 10] = [
    1,
    10,
    100,
    1000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
];

fn u32_from_bytes(bytes: &[u8]) -> u32 {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x - b'0') as u32 * POWERS_OF_TEN[ix]
    })
}

pub(crate) fn part1(input: &[u8]) -> u32 {
    input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .fold((0, 0), |(max_cals, cur_cals), bytes| {
            if bytes.is_empty() {
                (std::cmp::max(max_cals, cur_cals), 0)
            } else {
                (max_cals, cur_cals + u32_from_bytes(bytes))
            }
        })
        .0
}

pub(crate) fn part2(input: &[u8]) -> u32 {
    let mut top_three_plus_one = [0_u32; 4];

    input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .fold(0, |cur_cals, bytes| {
            if bytes.is_empty() {
                top_three_plus_one[0] = cur_cals;
                top_three_plus_one.sort_unstable();
                0
            } else {
                cur_cals + u32_from_bytes(bytes)
            }
        });

    top_three_plus_one.iter().skip(1).sum()
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
