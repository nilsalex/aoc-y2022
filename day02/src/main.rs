pub const INPUT: &[u8] = include_bytes!("input.txt");

const NEWLINE: u8 = 10;
const CAPITAL_A: u8 = 65;
const CAPITAL_Z: u8 = 88;

fn part1(input: &[u8]) -> u32 {
    input
        .split_inclusive(|byte| *byte == NEWLINE)
        .map(|line| {
            let a = line[0] - CAPITAL_A;
            let b = line[2] - CAPITAL_Z;
            (1 + b + ((4 + b - a) % 3) * 3) as u32 // add 4 instead of 1 because 4 + b - a is always > 0
        }).sum()
}

fn part2(input: &[u8]) -> u32 {
    input
        .split_inclusive(|byte| *byte == NEWLINE)
        .map(|line| {
            let a = line[0] - CAPITAL_A;
            let b = line[2] - CAPITAL_Z;
            (1 + 3 * b + (a + b + 2) % 3) as u32
        }).sum()
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
