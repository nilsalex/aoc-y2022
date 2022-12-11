#![feature(test)]
extern crate test;

use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    divisor: usize,
    next_if_divisible: usize,
    next_if_not_divisible: usize,
    num_items_inspected: usize,
}

fn solution(rounds: usize, calm_down_factor: usize) -> usize {
    let monkeys = vec![
        RefCell::new(Monkey {
            items: VecDeque::from([89, 73, 66, 57, 64, 80]),
            operation: Operation::Multiply(3),
            divisor: 13,
            next_if_divisible: 6,
            next_if_not_divisible: 2,
            num_items_inspected: 0,
        }),
        RefCell::new(Monkey {
            items: VecDeque::from([83, 78, 81, 55, 81, 59, 69]),
            operation: Operation::Add(1),
            divisor: 3,
            next_if_divisible: 7,
            next_if_not_divisible: 4,
            num_items_inspected: 0,
        }),
        RefCell::new(Monkey {
            items: VecDeque::from([76, 91, 58, 85]),
            operation: Operation::Multiply(13),
            divisor: 7,
            next_if_divisible: 1,
            next_if_not_divisible: 4,
            num_items_inspected: 0,
        }),
        RefCell::new(Monkey {
            items: VecDeque::from([71, 72, 74, 76, 68]),
            operation: Operation::Square,
            divisor: 2,
            next_if_divisible: 6,
            next_if_not_divisible: 0,
            num_items_inspected: 0,
        }),
        RefCell::new(Monkey {
            items: VecDeque::from([98, 85, 84]),
            operation: Operation::Add(7),
            divisor: 19,
            next_if_divisible: 5,
            next_if_not_divisible: 7,
            num_items_inspected: 0,
        }),
        RefCell::new(Monkey {
            items: VecDeque::from([78]),
            operation: Operation::Add(8),
            divisor: 5,
            next_if_divisible: 3,
            next_if_not_divisible: 0,
            num_items_inspected: 0,
        }),
        RefCell::new(Monkey {
            items: VecDeque::from([86, 70, 60, 88, 88, 78, 74, 83]),
            operation: Operation::Add(4),
            divisor: 11,
            next_if_divisible: 1,
            next_if_not_divisible: 2,
            num_items_inspected: 0,
        }),
        RefCell::new(Monkey {
            items: VecDeque::from([81, 58]),
            operation: Operation::Add(5),
            divisor: 17,
            next_if_divisible: 3,
            next_if_not_divisible: 5,
            num_items_inspected: 0,
        }),
    ];
    let modulus: usize = monkeys
        .iter()
        .map(|monkey| monkey.borrow().divisor)
        .product();

    for _ in 0..rounds {
        for monkey in &monkeys {
            let mut monkey = monkey.borrow_mut();
            while let Some(item) = monkey.items.pop_front() {
                monkey.num_items_inspected += 1;
                let new_item = (match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::Square => item * item,
                } / calm_down_factor)
                    % modulus;
                let next_monkey = if new_item % monkey.divisor == 0 {
                    monkey.next_if_divisible
                } else {
                    monkey.next_if_not_divisible
                };
                monkeys[next_monkey].borrow_mut().items.push_back(new_item)
            }
        }
    }

    let mut top_two_plus_one = [0, 0, 0];

    for monkey in monkeys {
        let monkey = monkey.borrow();
        top_two_plus_one[0] = monkey.num_items_inspected;
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
