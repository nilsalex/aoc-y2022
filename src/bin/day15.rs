#![feature(test)]
extern crate test;

use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day15.txt");

#[derive(Debug)]
struct Sensor {
    position: (isize, isize),
    nearest_beacon_position: (isize, isize),
    nearest_beacon_distance: isize,
}

impl Sensor {
    fn contains(&self, x: isize, y: isize) -> bool {
        dist(self.position.0, self.position.1, x, y) <= self.nearest_beacon_distance
    }

    fn left_most(&self) -> isize {
        self.position.0 - self.nearest_beacon_distance + 1
    }

    fn right_most(&self) -> isize {
        self.position.0 + self.nearest_beacon_distance - 1
    }
}

fn dist(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+).+x=(-?\d+), y=(-?\d+)$").unwrap();

    input
        .lines()
        .map(|line| {
            let mut captures_it = re.captures_iter(line);
            let capture = captures_it.next().unwrap();
            let position: (isize, isize) =
                (capture[1].parse().unwrap(), capture[2].parse().unwrap());
            let nearest_beacon_position: (isize, isize) =
                (capture[3].parse().unwrap(), capture[4].parse().unwrap());
            let nearest_beacon_distance = (nearest_beacon_position.0 - position.0).abs()
                + (nearest_beacon_position.1 - position.1).abs();
            Sensor {
                position,
                nearest_beacon_position,
                nearest_beacon_distance,
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let sensors = parse_sensors(input);

    let (left, right) = sensors
        .iter()
        .fold((isize::MAX, isize::MIN), |(left, right), sensor| {
            let (left_, right_) = (sensor.left_most(), sensor.right_most());
            (std::cmp::min(left, left_), std::cmp::max(right, right_))
        });

    let row: isize = 2000000;

    let mut x = left - 1;
    let mut result: usize = 0;

    while x <= right {
        if let Some(sensor) = sensors.iter().find(|sensor| sensor.contains(x, row)) {
            let vertical_dist = (sensor.position.1 - row).abs();
            let remaining_dist = sensor.nearest_beacon_distance - vertical_dist;
            let next_x = sensor.position.0 + remaining_dist + 1;
            result += (next_x - x) as usize;
            if sensor.nearest_beacon_position.1 == row
                && x <= sensor.nearest_beacon_position.0
                && next_x > sensor.nearest_beacon_position.0
            {
                result -= 1;
            }
            x = next_x;
        } else {
            x += 1;
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let sensors = parse_sensors(input);

    let max_dimensions: isize = 4000000;

    for y in 0..=max_dimensions {
        let mut x: isize = 0;

        while x <= max_dimensions {
            if let Some(sensor) = sensors.iter().find(|sensor| sensor.contains(x, y)) {
                let vertical_dist = (y - sensor.position.1).abs();
                let remaining_dist = sensor.nearest_beacon_distance - vertical_dist;
                x = sensor.position.0 + remaining_dist + 1;
            } else {
                return (4000000 * x + y) as usize;
            }
        }
    }

    panic!()
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
        assert_eq!(part1(INPUT), 5394423)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11840879211051)
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| parse_sensors(INPUT))
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
