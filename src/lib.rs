#![feature(test)]
#![feature(byte_slice_trim_ascii)]
#![feature(iter_intersperse)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn run_all() {
    println!("{}", day01::part1(day01::INPUT));
    println!("{}", day01::part2(day01::INPUT));

    println!("{}", day02::part1(day02::INPUT));
    println!("{}", day02::part2(day02::INPUT));

    println!("{}", day03::part1(day03::INPUT));
    println!("{}", day03::part2(day03::INPUT));

    println!("{}", day04::part1(day04::INPUT));
    println!("{}", day04::part2(day04::INPUT));

    println!("{}", std::str::from_utf8(&day05::part1(day05::INPUT)).unwrap());
    println!("{}", std::str::from_utf8(&day05::part2(day05::INPUT)).unwrap());

    println!("{}", day06::part1(day06::INPUT));
    println!("{}", day06::part2(day06::INPUT));

    println!("{}", day07::part1(day07::INPUT));
    println!("{}", day07::part2(day07::INPUT));

    println!("{}", day08::part1(day08::INPUT));
    println!("{}", day08::part2(day08::INPUT));

    println!("{}", day09::part1(day09::INPUT));
    println!("{}", day09::part2(day09::INPUT));

    println!("{}", day10::part1(day10::INPUT));
    println!("{}", day10::part2(day10::INPUT));

    println!("{}", day11::part1());
    println!("{}", day11::part2());

    println!("{}", day12::part1(day12::INPUT));
    println!("{}", day12::part2(day12::INPUT));

    println!("{}", day13::part1(day13::INPUT));
    println!("{}", day13::part2(day13::INPUT));

    println!("{}", day14::part1(day14::INPUT));
    println!("{}", day14::part2(day14::INPUT));

    println!("{}", day15::part1(day15::INPUT));
    println!("{}", day15::part2(day15::INPUT));

    println!("{}", day16::part1(day16::INPUT));
    println!("{}", day16::part2(day16::INPUT));

    println!("{}", day17::part1(day17::INPUT));
    println!("{}", day17::part2(day17::INPUT));

    println!("{}", day18::part1(day18::INPUT));
    println!("{}", day18::part2(day18::INPUT));

    println!("{}", day19::part1(day19::INPUT));
    println!("{}", day19::part2(day19::INPUT));

    println!("{}", day20::part1(day20::INPUT));
    println!("{}", day20::part2(day20::INPUT));

    println!("{}", day21::part1(day21::INPUT));
    println!("{}", day21::part2(day21::INPUT));

    println!("{}", day22::part1(day22::INPUT));
    println!("{}", day22::part2(day22::INPUT));

    println!("{}", day23::part1(day23::INPUT));
    println!("{}", day23::part2(day23::INPUT));

    println!("{}", day24::part1(day24::INPUT));
    println!("{}", day24::part2(day24::INPUT));

    println!("{}", std::str::from_utf8(&day25::part1(day25::INPUT)).unwrap());
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_all(b: &mut Bencher) {
        b.iter(|| run_all())
    }
}
