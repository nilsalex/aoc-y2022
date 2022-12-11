#![feature(test)]
extern crate test;

const MAX_ITEMS_PER_MONKEY: usize = 32;
const NUM_MONKEYS: usize = 8;
const ITEMS_OFFSET: usize = 7;

const MONKEY_SIZE: usize = MAX_ITEMS_PER_MONKEY + ITEMS_OFFSET;
const MONKEYS_SIZE: usize = NUM_MONKEYS * MONKEY_SIZE;

const MONKEYS: [usize; MONKEYS_SIZE] = [
    6, 3, 0, 13, 6, 2, 0, 89, 73, 66, 57, 64, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    7, 0, 1, 3, 7, 4, 0, 83, 78, 81, 55, 81, 59, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 13, 0, 7, 1, 4, 0, 76, 91, 58, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    5, 0, 0, 2, 6, 0, 0, 71, 72, 74, 76, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 7, 19, 5, 7, 0, 98, 85, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 8, 5, 3, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    8, 0, 4, 11, 1, 2, 0, 86, 70, 60, 88, 88, 78, 74, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    2, 0, 5, 17, 3, 5, 0, 81, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const MODULUS: usize = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;

const fn num_items_index(monkey: usize) -> usize {
    monkey * MONKEY_SIZE
}

const fn item_index(monkey: usize, item: usize) -> usize {
    monkey * MONKEY_SIZE + ITEMS_OFFSET + item
}

const fn factor_index(monkey: usize) -> usize {
    monkey * MONKEY_SIZE + 1
}

const fn summand_index(monkey: usize) -> usize {
    monkey * MONKEY_SIZE + 2
}

const fn divisor_index(monkey: usize) -> usize {
    monkey * MONKEY_SIZE + 3
}

const fn next_if_true_index(monkey: usize) -> usize {
    monkey * MONKEY_SIZE + 4
}

const fn next_if_false_index(monkey: usize) -> usize {
    monkey * MONKEY_SIZE + 5
}

const fn num_inspected_index(monkey: usize) -> usize {
    monkey * MONKEY_SIZE + 6
}

fn solution(rounds: usize, calm_down_factor: usize) -> usize {
    let mut monkeys = MONKEYS;

    for _ in 0..rounds {
        for monkey in 0..NUM_MONKEYS {
            for item in 0..monkeys[num_items_index(monkey)] {
                monkeys[num_inspected_index(monkey)] += 1;
                let mut worry_level = monkeys[item_index(monkey, item)];
                let factor = monkeys[factor_index(monkey)];
                let summand = monkeys[summand_index(monkey)];
                let divisor = monkeys[divisor_index(monkey)];
                if summand != 0 {
                    worry_level += summand;
                } else if factor != 0 {
                    worry_level *= factor;
                } else {
                    worry_level = worry_level * worry_level;
                }
                worry_level %= MODULUS;
                worry_level /= calm_down_factor;
                let next_monkey = if worry_level % divisor == 0 {
                    monkeys[next_if_true_index(monkey)]
                } else {
                    monkeys[next_if_false_index(monkey)]
                };
                let next_monkey_num_items = monkeys[num_items_index(next_monkey)];
                monkeys[item_index(next_monkey, next_monkey_num_items)] = worry_level;
                monkeys[num_items_index(next_monkey)] += 1;
            }
            monkeys[num_items_index(monkey)] = 0;
        }
    }

    let mut top_two_plus_one = [0, 0, 0];

    for monkey in 0..NUM_MONKEYS {
        top_two_plus_one[0] = monkeys[num_inspected_index(monkey)];
        top_two_plus_one.sort_unstable()
    }

    top_two_plus_one[1] * top_two_plus_one[2]
}

fn part1() -> usize {
    solution(20, 3)
}

fn part2() -> usize {
    solution(10000, 1)
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 119715)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 18085004878)
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1())
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2())
    }
}
