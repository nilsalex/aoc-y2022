pub const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> usize {
    let mut max_cals = 0;
    let mut cur_cals = 0;

    input
        .lines()
        .for_each(|l| {
            if l.is_empty() {
                if cur_cals > max_cals {
                    max_cals = cur_cals
                }
                cur_cals = 0
            } else {
                cur_cals += l.parse::<usize>().unwrap();
            }
        }
        );
    max_cals
}

fn part2(input: &str) -> usize {
    let mut sums: Vec<usize> = Vec::new();
    sums.push(0);

    input
        .lines()
        .for_each(|l| {
            if l.is_empty() {
                sums.push(0)
            } else {
                let cals: usize = l.parse().unwrap();
                *sums.last_mut().unwrap() += cals;
            }
        }
        );
    sums.sort_by(|a, b| b.cmp(a));
    sums[0] + sums[1] + sums[2]
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
