#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::collections::{HashSet, VecDeque};

const INPUT: &[u8] = include_bytes!("input.txt");

const POWERS_OF_TEN: [i8; 3] = [1, 10, 100];

fn i8_from_bytes(bytes: &[u8]) -> i8 {
    bytes
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, byte)| acc + (byte - b'0') as i8 * POWERS_OF_TEN[index])
}

fn parse_input(input: &[u8]) -> Vec<(i8, i8, i8)> {
    input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .map(|line| {
            let mut coords = line.split(|byte| *byte == b',');
            let x = i8_from_bytes(coords.next().unwrap());
            let y = i8_from_bytes(coords.next().unwrap());
            let z = i8_from_bytes(coords.next().unwrap());
            (x, y, z)
        })
        .collect()
}

fn neighbours(cube: &(i8, i8, i8)) -> Vec<(i8, i8, i8)> {
    let (x, y, z) = (cube.0, cube.1, cube.2);
    vec![(x - 1, y, z), (x + 1, y, z), (x, y - 1, z), (x, y + 1, z), (x, y, z - 1), (x, y, z + 1)]
}

fn part1(input: &[u8]) -> usize {
    let cubes = parse_input(input);
    let cubes_set: HashSet<(i8, i8, i8)> = HashSet::from_iter(cubes.iter().cloned());

    cubes
        .iter()
        .flat_map(neighbours)
        .filter(|cube| !cubes_set.contains(cube))
        .count()
}

fn part2(input: &[u8]) -> usize {
    let cubes = parse_input(input);

    let (x_min, x_max, y_min, y_max, z_min, z_max) = cubes
        .iter()
        .fold((i8::MAX, 0, i8::MAX, 0, i8::MAX, 0), |(x_min, x_max, y_min, y_max, z_min, z_max), (x, y, z)|
            (std::cmp::min(x_min, *x), std::cmp::max(x_max, *x),
             std::cmp::min(y_min, *y), std::cmp::max(y_max, *y),
             std::cmp::min(z_min, *z), std::cmp::max(z_max, *z)));

    let cubes_set: HashSet<(i8, i8, i8)> = HashSet::from_iter(cubes.iter().cloned());

    let mut visited: HashSet<(i8, i8, i8)> = HashSet::new();

    let mut queue: VecDeque<(i8, i8, i8)> = VecDeque::new();
    queue.push_back((x_min - 1, y_min - 1, z_min - 1));

    let mut surface_area: usize = 0;

    while let Some(cube) = queue.pop_front() {
        if visited.contains(&cube) {
            continue;
        }

        if cubes_set.contains(&cube) {
            surface_area += 1
        } else {
            visited.insert(cube);
            for next in neighbours(&cube) {
                if visited.contains(&next) {
                    continue;
                }
                if next.0 < x_min - 1 || next.1 < y_min - 1 || next.2 < z_min - 1 || next.0 > x_max + 1 || next.1 > y_max + 1 || next.2 > z_max + 1 {
                    continue;
                }
                queue.push_back(next);
            }
        }
    }

    surface_area
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
        assert_eq!(part1(INPUT), 3448)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2052)
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| parse_input(INPUT))
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
