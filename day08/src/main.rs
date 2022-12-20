#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::cmp::max;
use std::collections::HashSet;

// const INPUT: &[u8] = include_bytes!("input_test.txt");
const INPUT: &[u8] = include_bytes!("input.txt");

fn part1(input: &[u8]) -> usize {
    let grid: Vec<&[u8]> = input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .collect();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for i in 1..grid[0].len() - 1 {
        let mut cur_height = grid[0][i];

        for (j, _) in grid.iter().enumerate().skip(1) {
            let new_height = grid[j][i];
            if new_height > cur_height {
                cur_height = max(cur_height, new_height);
                visible.insert((j, i));
            }
        }

        cur_height = grid[grid.len() - 1][i];

        for (j, _) in grid.iter().enumerate().rev().skip(1) {
            let new_height = grid[j][i];
            if new_height > cur_height {
                cur_height = max(cur_height, new_height);
                visible.insert((j, i));
            }
        }
    }

    for i in 0..grid.len() {
        visible.insert((i, 0));
        visible.insert((i, grid.len() - 1));
        visible.insert((0, i));
        visible.insert((grid.len() - 1, i));
    }

    for i in 1..grid.len() - 1 {
        let mut cur_height = grid[i][0];

        for (j, _) in grid[i].iter().enumerate().skip(1) {
            let new_height = grid[i][j];
            if new_height > cur_height {
                cur_height = max(cur_height, new_height);
                visible.insert((i, j));
            }
        }

        cur_height = grid[i][grid.len() - 1];

        for (j, _) in grid[i].iter().enumerate().rev().skip(1) {
            let new_height = grid[i][j];
            if new_height > cur_height {
                cur_height = max(cur_height, new_height);
                visible.insert((i, j));
            }
        }
    }

    visible.len()
}

fn part2(input: &[u8]) -> usize {
    let grid: Vec<&[u8]> = input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .collect();

    let mut result = 0;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            let mut score = 1;
            let height = grid[i][j];

            for k in 1..grid[i].len() {
                if j < k {
                    score *= k - 1;
                    break;
                } else if grid[i][j - k] >= height {
                    score *= k;
                    break;
                }
            }

            for k in 1..grid[i].len() {
                if j + k > grid[i].len() - 1 {
                    score *= k - 1;
                    break;
                } else if grid[i][j + k] >= height {
                    score *= k;
                    break;
                }
            }

            for k in 1..grid.len() {
                if i < k {
                    score *= k - 1;
                    break;
                } else if grid[i - k][j] >= height {
                    score *= k;
                    break;
                }
            }

            for k in 1..grid.len() {
                if i + k > grid.len() - 1 {
                    score *= k - 1;
                    break;
                } else if grid[i + k][j] >= height {
                    score *= k;
                    break;
                }
            }

            // println!("score of {}, {}; {}", i, j, score);
            result = max(result, score)
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
        assert_eq!(part1(INPUT), 1763)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 671160)
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
