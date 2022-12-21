#![feature(test)]
extern crate test;

use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Monkey {
    Value(i64),
    Addition(String, String),
    Subtraction(String, String),
    Multiplication(String, String),
    Division(String, String),
}

impl Monkey {
    fn compute(&self, values: &HashMap<String, i64>) -> Option<i64> {
        match self {
            Monkey::Value(_) => {
                panic!()
            }
            Monkey::Addition(l1, l2) => {
                let m1 = values.get(l1)?;
                let m2 = values.get(l2)?;
                Some(m1 + m2)
            }
            Monkey::Subtraction(l1, l2) => {
                let m1 = values.get(l1)?;
                let m2 = values.get(l2)?;
                Some(m1 - m2)
            }
            Monkey::Multiplication(l1, l2) => {
                let m1 = values.get(l1)?;
                let m2 = values.get(l2)?;
                Some(m1 * m2)
            }
            Monkey::Division(l1, l2) => {
                let m1 = values.get(l1)?;
                let m2 = values.get(l2)?;
                Some(m1 / m2)
            }
        }
    }

    fn unapply(&self, values: &HashMap<String, i64>, v: i64) -> (i64, String) {
        match self {
            Monkey::Value(_) => {
                panic!()
            }
            Monkey::Addition(l1, l2) => {
                let v1 = values.get(l1);
                let v2 = values.get(l2);
                if let Some(v1) = v1 {
                    (v - *v1, l2.to_string())
                } else if let Some(v2) = v2 {
                    (v - *v2, l1.to_string())
                } else {
                    panic!()
                }
            }
            Monkey::Subtraction(l1, l2) => {
                let v1 = values.get(l1);
                let v2 = values.get(l2);
                if let Some(v1) = v1 {
                    (*v1 - v, l2.to_string())
                } else if let Some(v2) = v2 {
                    (*v2 + v, l1.to_string())
                } else {
                    panic!()
                }
            }
            Monkey::Multiplication(l1, l2) => {
                let v1 = values.get(l1);
                let v2 = values.get(l2);
                if let Some(v1) = v1 {
                    (v / *v1, l2.to_string())
                } else if let Some(v2) = v2 {
                    (v / *v2, l1.to_string())
                } else {
                    panic!()
                }
            }
            Monkey::Division(l1, l2) => {
                let v1 = values.get(l1);
                let v2 = values.get(l2);
                if let Some(v1) = v1 {
                    (*v1 / v, l2.to_string())
                } else if let Some(v2) = v2 {
                    (*v2 * v, l1.to_string())
                } else {
                    panic!()
                }
            }
        }
    }
}

fn parse_monkeys(
    input: &str,
) -> (
    Vec<Monkey>,
    HashMap<String, i64>,
    HashMap<String, usize>,
    Vec<String>,
) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_values: HashMap<String, i64> = HashMap::new();
    let mut monkey_positions: HashMap<String, usize> = HashMap::new();
    let mut unsolved_monkeys: Vec<String> = Vec::new();

    for line in input.trim_end().lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        let label = &words[0][0..4];
        monkey_positions.insert(label.to_string(), monkeys.len());
        if words.len() == 2 {
            let value = words[1].parse::<i64>().unwrap();
            monkeys.push(Monkey::Value(value));
            monkey_values.insert(label.to_string(), value);
        } else {
            let label_1 = String::from(words[1]);
            let label_2 = String::from(words[3]);
            let monkey = match words[2] {
                "+" => Monkey::Addition(label_1, label_2),
                "-" => Monkey::Subtraction(label_1, label_2),
                "*" => Monkey::Multiplication(label_1, label_2),
                "/" => Monkey::Division(label_1, label_2),
                _ => panic!(),
            };
            monkeys.push(monkey);
            unsolved_monkeys.push(label.to_string());
        };
    }

    (monkeys, monkey_values, monkey_positions, unsolved_monkeys)
}

fn part1(input: &str) -> i64 {
    let (monkeys, mut monkey_values, monkey_positions, mut unsolved_monkeys) = parse_monkeys(input);

    while !monkey_values.contains_key("root") {
        let mut new_unsolved_monkeys = Vec::new();

        for monkey in unsolved_monkeys {
            if let Some(value) =
                &monkeys[*monkey_positions.get(&monkey).unwrap()].compute(&monkey_values)
            {
                monkey_values.insert(monkey, *value);
            } else {
                new_unsolved_monkeys.push(monkey);
            }
        }

        unsolved_monkeys = new_unsolved_monkeys;
    }

    *monkey_values.get("root").unwrap()
}

fn part2(input: &str) -> i64 {
    let (monkeys, mut monkey_values, monkey_positions, mut unsolved_monkeys) = parse_monkeys(input);

    monkey_values.remove("humn");

    let root_monkey = &monkeys[*monkey_positions.get("root").unwrap()];
    let (r1, r2) = match root_monkey {
        Monkey::Addition(l1, l2) => (l1, l2),
        Monkey::Subtraction(l1, l2) => (l1, l2),
        Monkey::Multiplication(l1, l2) => (l1, l2),
        Monkey::Division(l1, l2) => (l1, l2),
        Monkey::Value(_) => panic!(),
    };

    loop {
        let mut new_unsolved_monkeys = Vec::new();

        for monkey in unsolved_monkeys.iter() {
            if let Some(value) =
                &monkeys[*monkey_positions.get(monkey).unwrap()].compute(&monkey_values)
            {
                monkey_values.insert(monkey.clone(), *value);
            } else {
                new_unsolved_monkeys.push(monkey.clone());
            }
        }

        if unsolved_monkeys.len() == new_unsolved_monkeys.len() {
            break;
        }

        unsolved_monkeys = new_unsolved_monkeys;
    }

    let (test_value, mut current_monkey) = if let Some(v) = monkey_values.get(r1) {
        (v, r2.clone())
    } else if let Some(v) = monkey_values.get(r2) {
        (v, r1.clone())
    } else {
        panic!()
    };

    let mut result = *test_value;

    while current_monkey != "humn" {
        let monkey = &monkeys[*monkey_positions.get(&current_monkey).unwrap()];
        let (next_value, next_monkey) = monkey.unapply(&monkey_values, result);
        current_monkey = next_monkey;
        result = next_value;
    }

    result
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 49288254556480)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 3558714869436)
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(INPUT))
    }
}
