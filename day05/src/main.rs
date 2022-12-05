#![feature(test)]
#![feature(byte_slice_trim_ascii)]
extern crate test;

const INPUT: &[u8] = include_bytes!("input.txt");
const POWERS_OF_TEN: [u8; 3] = [1, 10, 100];

fn bytes_to_u8(bytes: &[u8]) -> u8 {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| acc + (x - b'0') * POWERS_OF_TEN[ix])
}

fn parse_stacks(input: &[u8]) -> Vec<Vec<u8>> {
    let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); 9];
    for bytes in input.trim_ascii_end().split(|byte| *byte == b'\n') {
        if bytes[1] == b'1' {
            break;
        }
        for (index, byte) in bytes.trim_ascii_end().iter().skip(1).step_by(4).enumerate() {
            if *byte != b' ' {
                stacks[index].push(*byte)
            }
        }
    };
    for stack in stacks.iter_mut() {
        stack.reverse()
    }
    stacks
}

fn part1(input: &[u8]) -> Vec<u8> {
    let mut stacks: Vec<Vec<u8>> = parse_stacks(input);

    for bytes in input.trim_ascii_end().split(|byte| *byte == b'\n').skip(10) {
        let mut nums = bytes
            .split(|byte| *byte == b' ')
            .skip(1)
            .step_by(2)
            .map(|num| bytes_to_u8(num) as usize);

        let count = nums.next().unwrap();
        let src = nums.next().unwrap();
        let dst = nums.next().unwrap();

        for _ in 0..count {
            let to_move = stacks[src - 1].pop().unwrap();
            stacks[dst - 1].push(to_move)
        }
    }

    stacks.iter().map(|stack| *stack.last().unwrap()).collect()
}

fn part2(input: &[u8]) -> Vec<u8> {
    let mut stacks: Vec<Vec<u8>> = parse_stacks(input);
    let mut temp_stack: Vec<u8> = Vec::new();

    for bytes in input.trim_ascii_end().split(|byte| *byte == b'\n').skip(10) {
        let mut nums = bytes
            .split(|byte| *byte == b' ')
            .skip(1)
            .step_by(2)
            .map(|num| bytes_to_u8(num) as usize);

        let count = nums.next().unwrap();
        let src = nums.next().unwrap();
        let dst = nums.next().unwrap();

        for _ in 0..count {
            let to_move = stacks[src - 1].pop().unwrap();
            temp_stack.push(to_move);
        }

        for _ in 0..count {
            stacks[dst - 1].push(temp_stack.pop().unwrap())
        }
    }

    stacks.iter().map(|stack| *stack.last().unwrap()).collect()
}

fn main() {
    println!("{}", std::str::from_utf8(&part1(INPUT)).unwrap());
    println!("{}", std::str::from_utf8(&part2(INPUT)).unwrap());
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), b"VCTFTJQCG")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), b"GCFGLDNJZ")
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
