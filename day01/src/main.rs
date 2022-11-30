pub const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> usize {
    let mut count: usize = 0;
    let mut prev: usize = usize::MAX;
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .for_each(|x| {
            if x > prev {
                count += 1
            }
            prev = x
        }
        );
    count
}

fn part2(input: &str) -> usize {
    let values = input.lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut count: usize = 0;
    let mut prev: usize = usize::MAX;

    for i in 2..values.len() {
        let sum = values[i-2] + values[i-1] + values[i];
        if sum > prev {
            count += 1
        }
        prev = sum
    }

    count
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
