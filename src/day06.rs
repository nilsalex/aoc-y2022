extern crate test;

use std::collections::HashMap;

pub(crate) const INPUT: &[u8] = include_bytes!("../inputs/day06.txt");

pub(crate) fn part1(input: &[u8]) -> usize {
    input
        .windows(4)
        .position(|w| {
            w[0] != w[1]
                && w[0] != w[2]
                && w[0] != w[3]
                && w[1] != w[2]
                && w[1] != w[3]
                && w[2] != w[3]
        })
        .unwrap()
        + 4
}

pub(crate) fn part2(input: &[u8]) -> usize {
    let mut counts: HashMap<u8, u8> = HashMap::new();

    for item in &input[0..14] {
        match counts.get_mut(item) {
            None => {
                counts.insert(*item, 1);
            }
            Some(c) => {
                *c += 1;
            }
        }
    }

    let mut result = 14;
    let mut diff_count = counts.len();

    for window in input.windows(15) {
        if diff_count == 14 {
            break;
        }
        match counts.get_mut(&window[0]) {
            None => panic!(),
            Some(c) => {
                if *c > 1 {
                    *c -= 1;
                } else {
                    counts.remove(&window[0]);
                    diff_count -= 1;
                };
            }
        }
        match counts.get_mut(&window[14]) {
            None => {
                counts.insert(window[14], 1);
                diff_count += 1;
            }
            Some(c) => {
                *c += 1;
            }
        }
        result += 1
    }

    result
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1953)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2301)
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
