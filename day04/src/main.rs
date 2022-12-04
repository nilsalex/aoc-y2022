#![feature(test)]
extern crate test;

pub const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> usize {
    input.lines().filter(|line|
        {
            let mut pairs = line.split(',');
            let mut pair1 = pairs.next().unwrap().split('-');
            let mut pair2 = pairs.next().unwrap().split('-');

            let a = pair1.next().unwrap().parse::<usize>().unwrap();
            let b = pair1.next().unwrap().parse::<usize>().unwrap();

            let c = pair2.next().unwrap().parse::<usize>().unwrap();
            let d = pair2.next().unwrap().parse::<usize>().unwrap();

            a <= c && b >= d || c <= a && d >= b
        }
    )
        .count()
}

fn part2(input: &str) -> usize {
    input.lines().filter(|line|
        {
            let mut pairs = line.split(',');
            let mut pair1 = pairs.next().unwrap().split('-');
            let mut pair2 = pairs.next().unwrap().split('-');

            let a = pair1.next().unwrap().parse::<usize>().unwrap();
            let b = pair1.next().unwrap().parse::<usize>().unwrap();

            let c = pair2.next().unwrap().parse::<usize>().unwrap();
            let d = pair2.next().unwrap().parse::<usize>().unwrap();

            !(a < c && b < c || c < a && d < a)
        }
    )
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
