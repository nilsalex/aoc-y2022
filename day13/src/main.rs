#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::cmp::Ordering;

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Eq, PartialEq, Clone, Debug)]
enum Value {
    Integer(u8),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Value::Integer(left) => match other {
                Value::Integer(right) => left.cmp(right),
                Value::List(_) => Value::List(vec![self.clone()]).cmp(other),
            },
            Value::List(left) => match other {
                Value::Integer(_) => self.cmp(&Value::List(vec![other.clone()])),
                Value::List(right) => {
                    if left.is_empty() && right.is_empty() {
                        Ordering::Equal
                    } else if left.is_empty() {
                        Ordering::Less
                    } else if right.is_empty() {
                        Ordering::Greater
                    } else {
                        match left[0].cmp(&right[0]) {
                            Ordering::Less => Ordering::Less,
                            Ordering::Greater => Ordering::Greater,
                            Ordering::Equal => Value::List(Vec::from(&left[1..]))
                                .cmp(&Value::List(Vec::from(&right[1..]))),
                        }
                    }
                }
            },
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_closing(bytes: &[u8]) -> usize {
    let mut bracket_count = 0;
    let mut skipped = 0;

    for byte in bytes {
        skipped += 1;
        match byte {
            b'[' => {
                bracket_count += 1;
            }
            b']' => {
                bracket_count -= 1;
                if bracket_count == 0 {
                    return skipped;
                }
            }
            _ => {}
        }
    }
    println!("{}", std::str::from_utf8(bytes).unwrap());
    panic!();
}

fn parse_values(bytes: &[u8]) -> Vec<Value> {
    let mut values = Vec::new();
    let mut next_pos = 1;

    while next_pos < bytes.len() {
        match bytes[next_pos] {
            b']' => {
                break;
            }
            b'[' => {
                let to_skip = find_closing(&bytes[next_pos..]);
                let inner_values = parse_values(&bytes[next_pos..next_pos + to_skip]);
                values.push(Value::List(inner_values));
                next_pos += to_skip;
            }
            b',' => {
                next_pos += 1;
            }
            c => {
                values.push(Value::Integer(c));
                next_pos += 1;
            }
        }
    }

    values
}

fn part1(input: &[u8]) -> usize {
    let values: Vec<Vec<Value>> = input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(parse_values)
        .collect();

    let mut result = 0;

    for i in 0..values.len() / 2 {
        if values[2 * i] < values[2 * i + 1] {
            result += i + 1
        }
    }

    result
}

fn part2(input: &[u8]) -> usize {
    let mut values: Vec<Value> = input
        .trim_ascii_end()
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| Value::List(parse_values(line)))
        .collect();

    let divider_1 = Value::List(parse_values(b"[[2]]"));
    let divider_2 = Value::List(parse_values(b"[[6]]"));

    values.push(divider_1.clone());
    values.push(divider_2.clone());

    values.sort_unstable();

    (values.iter().position(|value| *value == divider_1).unwrap() + 1)
        * (values.iter().position(|value| *value == divider_2).unwrap() + 1)
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
        assert_eq!(part1(INPUT), 5330)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 27648)
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
