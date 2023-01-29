extern crate test;

pub(crate) const INPUT: &[u8] = include_bytes!("../inputs/day24.txt");
const MAX_NUM_ROWS: usize = 32;

const fn shift_bits_right_with_wrapping(bits: u128, num_bits: usize) -> u128 {
    (bits >> 1) | (bits & 1_u128) << (num_bits - 1)
}

const fn shift_bits_left_with_wrapping(bits: u128, num_bits: usize) -> u128 {
    let mask = 1_u128 << (num_bits - 1);
    let left_bits = bits & mask;
    let remaining_bits = bits & !mask;
    (remaining_bits << 1) | left_bits >> (num_bits - 1)
}

struct Grid {
    arrows_up: [u128; MAX_NUM_ROWS],
    arrows_down: [u128; MAX_NUM_ROWS],
    arrows_left: [u128; MAX_NUM_ROWS],
    arrows_right: [u128; MAX_NUM_ROWS],
    positions: [u128; MAX_NUM_ROWS],
    start_bit: bool,
    end_bit: bool,
    num_rows: usize,
    num_cols: usize,
}

impl Grid {
    fn parse(input: &[u8]) -> Self {
        let mut arrows_up = [0_u128; MAX_NUM_ROWS];
        let mut arrows_down = [0_u128; MAX_NUM_ROWS];
        let mut arrows_left = [0_u128; MAX_NUM_ROWS];
        let mut arrows_right = [0_u128; MAX_NUM_ROWS];
        let positions = [0_u128; MAX_NUM_ROWS];

        let num_cols = input.iter().position(|byte| *byte == b'\n').unwrap() - 2;
        let num_rows = input.trim_ascii_end().split(|byte| *byte == b'\n').count() - 2;

        input
            .trim_ascii_end()
            .split(|byte| *byte == b'\n')
            .skip(1)
            .take(num_rows)
            .enumerate()
            .for_each(|(row, line)| {
                {
                    line.iter().skip(1).take(num_cols).enumerate().for_each(
                        |(col, byte)| match byte {
                            b'^' => arrows_up[row] |= 1_u128 << col,
                            b'v' => arrows_down[row] |= 1_u128 << col,
                            b'<' => arrows_left[row] |= 1_u128 << col,
                            b'>' => arrows_right[row] |= 1_u128 << col,
                            b'.' => {}
                            _ => panic!(),
                        },
                    )
                }
            });

        Self {
            arrows_up,
            arrows_down,
            arrows_left,
            arrows_right,
            positions,
            start_bit: false,
            end_bit: false,
            num_rows,
            num_cols,
        }
    }

    fn step(&mut self) {
        self.arrows_up[0..self.num_rows].rotate_left(1);
        self.arrows_down[0..self.num_rows].rotate_right(1);

        for col in self.arrows_left.iter_mut() {
            *col = shift_bits_right_with_wrapping(*col, self.num_cols);
        }
        for col in self.arrows_right.iter_mut() {
            *col = shift_bits_left_with_wrapping(*col, self.num_cols);
        }

        let previous_start = self.start_bit;
        let previous_end = self.end_bit;

        if self.positions[0] & 1_u128 != 0_u128 {
            self.start_bit = true
        }

        if self.positions[self.num_rows - 1] & (1_u128 << (self.num_cols - 1)) != 0_u128 {
            self.end_bit = true
        }

        let mut previous_col = 0_u128;
        for row in 0..self.num_rows {
            let current_col = self.positions[row];
            self.positions[row] |= previous_col | (current_col >> 1) | (current_col << 1);
            previous_col = current_col;
            if row + 1 < self.num_rows {
                self.positions[row] |= self.positions[row + 1]
            }
            self.positions[row] &= !(1_u128 << self.num_cols);
            self.positions[row] &= !(self.arrows_up[row]
                | self.arrows_down[row]
                | self.arrows_left[row]
                | self.arrows_right[row])
        }

        if previous_start {
            self.positions[0] |= 1_u128
        }

        if previous_end {
            self.positions[self.num_rows - 1] |= 1_u128 << (self.num_cols - 1)
        }
    }
}

pub(crate) fn part1(input: &[u8]) -> usize {
    let mut grid = Grid::parse(input);
    grid.start_bit = true;

    let mut time = 0;
    while !grid.end_bit {
        time += 1;
        grid.step();
    }

    time
}

pub(crate) fn part2(input: &[u8]) -> usize {
    let mut grid = Grid::parse(input);
    let mut time = 0;

    grid.start_bit = true;
    while !grid.end_bit {
        time += 1;
        grid.step();
    }

    grid.positions.fill(0);
    grid.start_bit = false;
    grid.end_bit = true;
    while !grid.start_bit {
        time += 1;
        grid.step();
    }

    grid.positions.fill(0);
    grid.start_bit = true;
    grid.end_bit = false;
    while !grid.end_bit {
        time += 1;
        grid.step();
    }

    time
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_shift_right() {
        let bits: u128 = 0b0000000010000101;
        assert_eq!(shift_bits_right_with_wrapping(bits, 8), 0b0000000011000010);
    }

    #[test]
    fn test_shift_left() {
        let bits: u128 = 0b0000000010000101;
        assert_eq!(shift_bits_left_with_wrapping(bits, 8), 0b0000000000001011);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 308)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 908)
    }

    #[bench]
    fn bench_parse_grid(b: &mut Bencher) {
        b.iter(|| Grid::parse(INPUT))
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
