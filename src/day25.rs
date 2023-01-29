extern crate test;

pub(crate) const INPUT: &[u8] = include_bytes!("../inputs/day25.txt");

const SNAFU_DIGIT_ARRAY: [isize; 62] = build_snafu_digit_array();
const INVERTED_SNAFU_DIGIT_ARRAY: [u8; 5] = *b"=-012";

const fn build_snafu_digit_array() -> [isize; 62] {
    let mut array = [0; 62];

    array[b'2' as usize] = 2_isize;
    array[b'1' as usize] = 1_isize;
    array[b'0' as usize] = 0_isize;
    array[b'-' as usize] = -1_isize;
    array[b'=' as usize] = -2_isize;

    array
}

const fn from_snafu(snafu: &[u8]) -> isize {
    let mut index: usize = 0;
    let mut result: isize = 0;

    while index < snafu.len() {
        result *= 5;
        result += SNAFU_DIGIT_ARRAY[snafu[index] as usize];
        index += 1
    }

    result
}

const fn to_snafu(decimal: isize) -> [u8; 26] {
    let mut snafu = [b'0'; 26];
    let mut index = 25;
    let mut decimal = decimal;

    while decimal != 0 {
        let modulus = match decimal % 5 {
            m @ 3..=4 => m - 5,
            m => m,
        };
        snafu[index] = INVERTED_SNAFU_DIGIT_ARRAY[(modulus + 2) as usize];
        decimal -= modulus;
        decimal /= 5;
        index -= 1;
    }

    snafu
}

pub(crate) fn part1(input: &[u8]) -> [u8; 26] {
    to_snafu(
        input
            .split(|byte| *byte == b'\n')
            .map(from_snafu)
            .sum::<isize>(),
    )
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), *b"0000002-121-=10=200==2==21")
    }

    #[bench]
    fn bench_to_snafu(b: &mut Bencher) {
        b.iter(|| to_snafu(123456789))
    }

    #[bench]
    fn bench_sum(b: &mut Bencher) {
        b.iter(|| {
            INPUT
                .split(|byte| *byte == b'\n')
                .map(from_snafu)
                .sum::<isize>();
        })
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT))
    }
}
