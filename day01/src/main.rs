pub const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> usize {
    input.lines().map(|l| l.parse::<usize>().unwrap()).sum()
}

fn part2(input: &str) -> usize {
    input.lines().map(|l| l.parse::<usize>().unwrap()).sum()
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
