extern crate test;

const MAX_ITEMS_PER_MONKEY: usize = 32;
const NUM_MONKEYS: usize = 8;
const MODULUS: usize = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;

const fn item_index(monkey: usize, item: usize) -> usize {
    monkey * MAX_ITEMS_PER_MONKEY + item
}

fn solution(rounds: usize, calm_down_factor: usize) -> usize {
    let mut nums: [usize; 8] = [6, 7, 4, 5, 3, 1, 8, 2];
    let factors: [usize; 8] = [3, 0, 13, 0, 0, 0, 0, 0];
    let summands: [usize; 8] = [0, 1, 0, 0, 7, 8, 4, 5];
    let divisors: [usize; 8] = [13, 3, 7, 2, 19, 5, 11, 17];
    let next_true: [usize; 8] = [6, 7, 1, 6, 5, 3, 1, 3];
    let next_false: [usize; 8] = [2, 4, 4, 0, 7, 0, 2, 5];
    let mut num_inspections: [usize; 8] = [0; 8];

    let mut items: [usize; NUM_MONKEYS * MAX_ITEMS_PER_MONKEY] = [
        89, 73, 66, 57, 64, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 83, 78, 81, 55, 81, 59, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 76, 91, 58, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 72, 74, 76, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 85, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 86, 70, 60, 88, 88, 78, 74, 83, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 58, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    for _ in 0..rounds {
        for monkey in 0..NUM_MONKEYS {
            for item in 0..nums[monkey] {
                num_inspections[monkey] += 1;
                let mut worry_level = items[item_index(monkey, item)];
                let factor = factors[monkey];
                let summand = summands[monkey];
                let divisor = divisors[monkey];
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
                    next_true[monkey]
                } else {
                    next_false[monkey]
                };
                let next_monkey_num_items = nums[next_monkey];
                items[item_index(next_monkey, next_monkey_num_items)] = worry_level;
                nums[next_monkey] += 1;
            }
            nums[monkey] = 0;
        }
    }

    num_inspections.sort();

    num_inspections[6] * num_inspections[7]
}

pub(crate) fn part1() -> usize {
    solution(20, 3)
}

pub(crate) fn part2() -> usize {
    solution(10000, 1)
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
