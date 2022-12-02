#![feature(test)]
extern crate test;

pub const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut cals = input
        .split("\n\n")
        .map(|group| group.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect::<Vec<usize>>();
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
