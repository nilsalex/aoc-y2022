#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part1(input: &[u8]) -> isize {
    const CYCLES: [u32; 6] = [20, 60, 100, 140, 180, 220];
    let mut register: isize = 1;
    let mut cycle: u32 = 1;
    let mut result: isize = 0;

    for instruction in input.trim_ascii_end().split(|byte| *byte == b'\n') {
        if CYCLES.contains(&cycle) {
            result += (cycle as isize) * register;
        }
        if instruction[0] == b'n' {
            cycle += 1;
        } else {
            let v = std::str::from_utf8(&instruction[5..])
                .unwrap()
                .parse::<isize>()
                .unwrap();
            cycle += 1;
            if CYCLES.contains(&cycle) {
                result += (cycle as isize) * register;
            }
            cycle += 1;
            register += v;
        }
    }

    result
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
            let v = std::str::from_utf8(&instruction[5..])
                .unwrap()
                .parse::<isize>()
                .unwrap();
            cycle = (cycle + 1) % 240;
            crt_row = cycle / 40;
            crt_col = cycle - crt_row * 40;
            result[crt_row][crt_col] = ((crt_col as isize) - register).abs() < 2;
            cycle = (cycle + 1) % 240;
            crt_row = cycle / 40;
            crt_col = cycle - crt_row * 40;
            register += v;
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
