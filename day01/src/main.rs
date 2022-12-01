pub const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut cals = input
        .split("\n\n")
        .map(|group| group.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect::<Vec<usize>>();
    cals.select_nth_unstable_by(3, |a,b| b.cmp(a));
    cals.iter().take(3).sum()
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
