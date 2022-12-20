#![feature(test)]
extern crate test;

use std::cmp::Ordering;

const INPUT: &str = include_str!("input.txt");

fn solution(input: &str, decryption_key: isize, cycles: usize) -> isize {
    let numbers = input
        .trim_end()
        .lines()
        .map(|line| line.parse::<isize>().unwrap() * decryption_key)
        .collect::<Vec<isize>>();
    let modulus = numbers.len() - 1;

    let mut decrypted: Vec<(usize, isize)> = numbers.iter().cloned().enumerate().collect();

    for _ in 0..cycles {
        for (index, number) in numbers.iter().cloned().enumerate() {
            let position = decrypted.iter().position(|(k, _)| *k == index).unwrap();

            if decrypted[position].1 != number {
                panic!();
            }

            let mut new_position =
                (position as isize + number).rem_euclid(modulus as isize) as usize;

            if new_position == 0 {
                new_position = modulus;
            }

            match new_position.cmp(&position) {
                Ordering::Less => {
                    let old_value = decrypted[position];
                    for i in (new_position..position).rev() {
                        decrypted[i + 1] = decrypted[i];
                    }
                    decrypted[new_position] = old_value;
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    let old_value = decrypted[position];
                    for i in position..new_position {
                        decrypted[i] = decrypted[i + 1];
                    }
                    decrypted[new_position] = old_value;
                }
            }
        }
    }

    let result = decrypted.iter().map(|(_, v)| *v).collect::<Vec<isize>>();
    let position_zero = result.iter().position(|x| *x == 0).unwrap();

    result[(position_zero + 1000).rem_euclid(numbers.len())]
        + result[(position_zero + 2000).rem_euclid(numbers.len())]
        + result[(position_zero + 3000).rem_euclid(numbers.len())]
}

fn part1(input: &str) -> isize {
    solution(input, 1, 1)
}

fn part2(input: &str) -> isize {
    solution(input, 811589153, 10)
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
        assert_eq!(part1(INPUT), 6640)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11893839037215)
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
