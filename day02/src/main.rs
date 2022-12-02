pub const INPUT: &str = include_str!("input.txt");

const CAPITAL_A: u8 = 65;
const CAPITAL_Z: u8 = 88;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let bytes = l.as_bytes();
            let a = bytes[0] - CAPITAL_A;
            let b = bytes[2] - CAPITAL_Z;
            (1 + b + ((4 + b - a) % 3) * 3) as u32 // add 4 instead of 1 because 4 + b - a is always > 0
        }).sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let bytes = l.as_bytes();
            let a = bytes[0] - CAPITAL_A;
            let b = bytes[2] - CAPITAL_Z;
            (1 + 3 * b + (a + b + 2) % 3) as u32
        }).sum()
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
