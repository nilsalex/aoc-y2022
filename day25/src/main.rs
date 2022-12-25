#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

const INPUT: &[u8] = include_bytes!("input.txt");

fn from_snafu(snafu: &[u8]) -> isize {
    snafu
        .iter()
        .rev()
        .map(|c| match c {
            b'2' => 2_isize,
            b'1' => 1_isize,
            b'0' => 0_isize,
            b'-' => -1_isize,
            b'=' => -2_isize,
            _ => panic!(),
        })
        .enumerate()
        .fold(0_isize, |acc, (i, x)| acc + 5_isize.pow(i as u32) * x)
}

fn to_snafu(x: isize) -> Vec<u8> {
    let mut x = x;
    let mut digits: Vec<u8> = Vec::new();

    while x != 0 {
        let modulus = match x.rem_euclid(5) {
            m @ 3..=4 => m - 5,
            m => m,
        };
        let digit = match modulus {
            -2 => b'=',
            -1 => b'-',
            0 => b'0',
            1 => b'1',
            2 => b'2',
            _ => panic!(),
        };
        digits.push(digit);
        x -= modulus;
        x /= 5;
    }

    digits.reverse();

    digits
}

fn part1(input: &[u8]) -> Vec<u8> {
    let sum = input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .map(from_snafu)
        .sum();

    to_snafu(sum)
}

fn main() {
    println!("{}", std::str::from_utf8(&part1(INPUT)).unwrap());
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(&part1(INPUT), b"2-121-=10=200==2==21")
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT))
    }
}
