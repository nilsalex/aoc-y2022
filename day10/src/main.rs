#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

const INPUT: &[u8] = include_bytes!("input.txt");

const POWERS_OF_TEN: [i8; 3] = [1, 10, 100];

fn i8_from_bytes(bytes: &[u8]) -> i8 {
    if bytes[0] == b'-' {
        bytes
            .iter()
            .skip(1)
            .rev()
            .enumerate()
            .fold(0, |acc, (ix, x)| acc + (48 - *x as i8) * POWERS_OF_TEN[ix])
    } else {
        bytes
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (ix, x)| acc + (*x as i8 - 48) * POWERS_OF_TEN[ix])
    }
}

fn part1(input: &[u8]) -> isize {
    input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .fold(
            (1_isize, 1_isize, 0_isize),
            |(cycle, register, signal), instruction| {
                let (mut new_cycle, mut new_signal) = (cycle, signal);
                if (new_cycle + 20) % 40 == 0 {
                    new_signal += new_cycle * register;
                }
                new_cycle += 1;
                if instruction[0] == b'n' {
                    (new_cycle, register, new_signal)
                } else {
                    if (new_cycle + 20) % 40 == 0 {
                        new_signal += new_cycle * register;
                    }
                    (
                        new_cycle + 1,
                        register + i8_from_bytes(&instruction[5..]) as isize,
                        new_signal,
                    )
                }
            },
        )
        .2
}

fn part2(input: &[u8]) -> String {
    let mut register: isize = 1;
    let mut cycle: usize = 0;
    let mut crt_row: usize;
    let mut crt_col: usize;
    let mut result = vec![vec![false; 40]; 6];

    for instruction in input.trim_ascii_end().split(|byte| *byte == b'\n') {
        if instruction[0] == b'n' {
            cycle = (cycle + 1) % 240;
            crt_row = cycle / 40;
            crt_col = cycle - crt_row * 40;
            result[crt_row][crt_col] = ((crt_col as isize) - register).abs() < 2;
        } else {
            cycle = (cycle + 1) % 240;
            crt_row = cycle / 40;
            crt_col = cycle - crt_row * 40;
            result[crt_row][crt_col] = ((crt_col as isize) - register).abs() < 2;
            cycle = (cycle + 1) % 240;
            crt_row = cycle / 40;
            crt_col = cycle - crt_row * 40;
            register += i8_from_bytes(&instruction[5..]) as isize;
            result[crt_row][crt_col] = ((crt_col as isize) - register).abs() < 2;
        }
    }

    let mut out = String::new();
    for row in result {
        for pixel in row {
            out.push(if pixel { '#' } else { '.' });
        }
        out.push('\n');
    }
    out
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
        assert_eq!(part1(INPUT), 13220)
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(INPUT),
            r"
.##..#..#..##..#..#.#..#.###..####.#..#.
#..#.#..#.#..#.#.#..#..#.#..#.#....#.#..
#..#.#..#.#..#.##...####.###..###..##...
###..#..#.####.#.#..#..#.#..#.#....#.#..
#.#..#..#.#..#.#.#..#..#.#..#.#....#.#..
#..#..##..#..#.#..#.#..#.###..####.#..#.
"
            .trim_start()
        )
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
