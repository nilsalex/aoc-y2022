#![feature(test)]
extern crate test;

use std::cmp::max;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> (HashSet<(u32, u32)>, u32) {
    let mut occupied: HashSet<(u32, u32)> = HashSet::new();
    let mut max_y: u32 = 0;

    for line in input.lines() {
        let nodes = line.split(" -> ").map(|coords| {
            let mut split = coords.split(',');
            let x = split.next().unwrap().parse::<i32>().unwrap();
            let y = split.next().unwrap().parse::<i32>().unwrap();
            (x, y)
        }).collect::<Vec<(i32, i32)>>();
        for window in nodes.windows(2) {
            let start = window[0];
            let end = window[1];
            let (dx, dy) = (end.0 - start.0, end.1 - start.1);

            let steps = max(dx.abs(), dy.abs());
            let (dx_, dy_) = if dx == 0 && dy == 0 {
                (0, 0)
            } else {
                (dx / steps, dy / steps)
            };

            for i in 0..=steps {
                let (x, y) = (start.0 + dx_ * i, start.1 + dy_ * i);
                occupied.insert((x as u32, y as u32));
            }
            max_y = max(max_y, start.1 as u32);
            max_y = max(max_y, end.1 as u32);
        }
    }

    (occupied, max_y)
}

fn next(sx: u32, sy: u32, occupied: &HashSet<(u32, u32)>) -> Option<(u32, u32)> {
    if !occupied.contains(&(sx, sy + 1)) {
        Some((sx, sy + 1))
    } else if !occupied.contains(&(sx - 1, sy + 1)) {
        Some((sx - 1, sy + 1))
    } else if !occupied.contains(&(sx + 1, sy + 1)) {
        Some((sx + 1, sy + 1))
    } else {
        None
    }
}

fn part1(input: &str) -> usize {
    let (mut occupied, max_y) = parse_input(input);

    let mut result = 0;

    'outer: loop {
        let (mut sx, mut sy) = (500, 0);

        while let Some((sx_, sy_)) = next(sx, sy, &occupied) {
            if sy > max_y {
                break 'outer;
            }
            (sx, sy) = (sx_, sy_);
        }

        occupied.insert((sx, sy));
        result += 1;
    }

    result
}

fn part2(input: &str) -> usize {
    let (mut occupied, max_y) = parse_input(input);

    let mut result = 0;

    loop {
        let (mut sx, mut sy) = (500, 0);

        while let Some((sx_, sy_)) = next(sx, sy, &occupied) {
            if sy_ == max_y + 2 {
                break;
            }
            (sx, sy) = (sx_, sy_);
        }

        occupied.insert((sx, sy));
        result += 1;

        if sy == 0 && sx == 500 {
            break;
        }
    }

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
        assert_eq!(part1(INPUT), 757)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 24943)
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
