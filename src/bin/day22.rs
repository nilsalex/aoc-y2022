#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

const INPUT: &[u8] = include_bytes!("../../inputs/day22.txt");

const CUBE_SIZE: usize = 50;
const NUM_FACES: usize = 6;

const FACES_2D: [Face; NUM_FACES] = [
    Face {
        id: 0,
        position: (0, CUBE_SIZE),
        next_up: (4, Direction::U, Orientation::Normal),
        next_down: (2, Direction::D, Orientation::Normal),
        next_left: (1, Direction::L, Orientation::Normal),
        next_right: (1, Direction::R, Orientation::Normal),
    },
    Face {
        id: 1,
        position: (0, 2 * CUBE_SIZE),
        next_up: (1, Direction::U, Orientation::Normal),
        next_down: (1, Direction::D, Orientation::Normal),
        next_left: (0, Direction::L, Orientation::Normal),
        next_right: (0, Direction::R, Orientation::Normal),
    },
    Face {
        id: 2,
        position: (CUBE_SIZE, CUBE_SIZE),
        next_up: (0, Direction::U, Orientation::Normal),
        next_down: (4, Direction::D, Orientation::Normal),
        next_left: (2, Direction::L, Orientation::Normal),
        next_right: (2, Direction::R, Orientation::Normal),
    },
    Face {
        id: 3,
        position: (2 * CUBE_SIZE, 0),
        next_up: (5, Direction::U, Orientation::Normal),
        next_down: (5, Direction::D, Orientation::Normal),
        next_left: (4, Direction::L, Orientation::Normal),
        next_right: (4, Direction::R, Orientation::Normal),
    },
    Face {
        id: 4,
        position: (2 * CUBE_SIZE, CUBE_SIZE),
        next_up: (2, Direction::U, Orientation::Normal),
        next_down: (0, Direction::D, Orientation::Normal),
        next_left: (3, Direction::L, Orientation::Normal),
        next_right: (3, Direction::R, Orientation::Normal),
    },
    Face {
        id: 5,
        position: (3 * CUBE_SIZE, 0),
        next_up: (3, Direction::U, Orientation::Normal),
        next_down: (3, Direction::D, Orientation::Normal),
        next_left: (5, Direction::L, Orientation::Normal),
        next_right: (5, Direction::R, Orientation::Normal),
    },
];

const FACES_3D: [Face; NUM_FACES] = [
    Face {
        id: 0,
        position: (0, CUBE_SIZE),
        next_up: (5, Direction::R, Orientation::Normal),
        next_down: (2, Direction::D, Orientation::Normal),
        next_left: (3, Direction::R, Orientation::Flipped),
        next_right: (1, Direction::R, Orientation::Normal),
    },
    Face {
        id: 1,
        position: (0, 2 * CUBE_SIZE),
        next_up: (5, Direction::U, Orientation::Normal),
        next_down: (2, Direction::L, Orientation::Normal),
        next_left: (0, Direction::L, Orientation::Normal),
        next_right: (4, Direction::L, Orientation::Flipped),
    },
    Face {
        id: 2,
        position: (CUBE_SIZE, CUBE_SIZE),
        next_up: (0, Direction::U, Orientation::Normal),
        next_down: (4, Direction::D, Orientation::Normal),
        next_left: (3, Direction::D, Orientation::Normal),
        next_right: (1, Direction::U, Orientation::Normal),
    },
    Face {
        id: 3,
        position: (2 * CUBE_SIZE, 0),
        next_up: (2, Direction::R, Orientation::Normal),
        next_down: (5, Direction::D, Orientation::Normal),
        next_left: (0, Direction::R, Orientation::Flipped),
        next_right: (4, Direction::R, Orientation::Normal),
    },
    Face {
        id: 4,
        position: (2 * CUBE_SIZE, CUBE_SIZE),
        next_up: (2, Direction::U, Orientation::Normal),
        next_down: (5, Direction::L, Orientation::Normal),
        next_left: (3, Direction::L, Orientation::Normal),
        next_right: (1, Direction::L, Orientation::Flipped),
    },
    Face {
        id: 5,
        position: (3 * CUBE_SIZE, 0),
        next_up: (3, Direction::U, Orientation::Normal),
        next_down: (1, Direction::D, Orientation::Normal),
        next_left: (0, Direction::D, Orientation::Normal),
        next_right: (4, Direction::U, Orientation::Normal),
    },
];

const POWERS_OF_TEN: [usize; 2] = [1, 10];

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, c)| {
        acc + (c - b'0') as usize * POWERS_OF_TEN[ix]
    })
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Fwd(usize),
    L,
    R,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    U,
    L,
    R,
    D,
}

impl Direction {
    fn turn(&self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Fwd(_) => panic!(),
            Instruction::L => match self {
                Direction::U => Self::L,
                Direction::L => Self::D,
                Direction::R => Self::U,
                Direction::D => Self::R,
            },
            Instruction::R => match self {
                Direction::U => Self::R,
                Direction::L => Self::U,
                Direction::R => Self::D,
                Direction::D => Self::L,
            },
        }
    }

    fn score(&self) -> usize {
        match self {
            Direction::U => 3,
            Direction::L => 2,
            Direction::R => 0,
            Direction::D => 1,
        }
    }
}

#[derive(Debug)]
enum Orientation {
    Normal,
    Flipped,
}

#[derive(Debug)]
struct Face {
    id: usize,
    position: (usize, usize),
    next_up: (usize, Direction, Orientation),
    next_down: (usize, Direction, Orientation),
    next_left: (usize, Direction, Orientation),
    next_right: (usize, Direction, Orientation),
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Open,
    Wall,
}

enum AfterStep {
    HitWall,
    Proceed(usize, Direction, usize, usize),
}

struct Grid {
    cells: [Cell; NUM_FACES * CUBE_SIZE * CUBE_SIZE],
}

impl Grid {
    fn set_cell_as_wall(&mut self, face_id: usize, row: usize, col: usize) {
        self.cells[face_id * CUBE_SIZE * CUBE_SIZE + row * CUBE_SIZE + col] = Cell::Wall
    }

    fn get_cell(&self, face_id: usize, row: usize, col: usize) -> Cell {
        self.cells[face_id * CUBE_SIZE * CUBE_SIZE + row * CUBE_SIZE + col]
    }

    fn fwd(
        &self,
        faces: &[Face],
        dir: &Direction,
        face: &Face,
        row: usize,
        col: usize,
    ) -> AfterStep {
        let (next_face, next_dir, next_row, next_col) = match dir {
            Direction::U => {
                if row > 0 {
                    (face, dir, row - 1, col)
                } else {
                    let next_face = &faces[face.next_up.0];
                    let next_dir = &face.next_up.1;
                    let col = if let Orientation::Flipped = face.next_up.2 {
                        CUBE_SIZE - col - 1
                    } else {
                        col
                    };
                    let (next_row, next_col) = Self::pos_on_next_face(next_dir, col);
                    (next_face, next_dir, next_row, next_col)
                }
            }
            Direction::L => {
                if col > 0 {
                    (face, dir, row, col - 1)
                } else {
                    let next_face = &faces[face.next_left.0];
                    let next_dir = &face.next_left.1;
                    let row = if let Orientation::Flipped = face.next_left.2 {
                        CUBE_SIZE - row - 1
                    } else {
                        row
                    };
                    let (next_row, next_col) = Self::pos_on_next_face(next_dir, row);
                    (next_face, next_dir, next_row, next_col)
                }
            }
            Direction::R => {
                if col < CUBE_SIZE - 1 {
                    (face, dir, row, col + 1)
                } else {
                    let next_face = &faces[face.next_right.0];
                    let next_dir = &face.next_right.1;
                    let row = if let Orientation::Flipped = face.next_right.2 {
                        CUBE_SIZE - row - 1
                    } else {
                        row
                    };
                    let (next_row, next_col) = Self::pos_on_next_face(next_dir, row);
                    (next_face, next_dir, next_row, next_col)
                }
            }
            Direction::D => {
                if row < CUBE_SIZE - 1 {
                    (face, dir, row + 1, col)
                } else {
                    let next_face = &faces[face.next_down.0];
                    let next_dir = &face.next_down.1;
                    let col = if let Orientation::Flipped = face.next_down.2 {
                        CUBE_SIZE - col - 1
                    } else {
                        col
                    };
                    let (next_row, next_col) = Self::pos_on_next_face(next_dir, col);
                    (next_face, next_dir, next_row, next_col)
                }
            }
        };
        match self.get_cell(next_face.id, next_row, next_col) {
            Cell::Open => AfterStep::Proceed(next_face.id, *next_dir, next_row, next_col),
            Cell::Wall => AfterStep::HitWall,
        }
    }

    fn pos_on_next_face(dir: &Direction, pos_on_border: usize) -> (usize, usize) {
        match dir {
            Direction::U => (CUBE_SIZE - 1, pos_on_border),
            Direction::L => (pos_on_border, CUBE_SIZE - 1),
            Direction::R => (pos_on_border, 0),
            Direction::D => (0, pos_on_border),
        }
    }

    fn parse(input: &[u8]) -> Self {
        let mut grid = Grid {
            cells: [Cell::Open; NUM_FACES * CUBE_SIZE * CUBE_SIZE],
        };

        let mut lines_it = input.split(|byte| *byte == b'\n');

        for (row, line) in lines_it.by_ref().enumerate().take(CUBE_SIZE) {
            let mut line_it = line.iter();
            for (column, cell) in line_it.by_ref().skip(CUBE_SIZE).enumerate().take(CUBE_SIZE) {
                if *cell == b'#' {
                    grid.set_cell_as_wall(0, row, column);
                }
            }
            for (column, cell) in line_it.by_ref().enumerate().take(CUBE_SIZE) {
                if *cell == b'#' {
                    grid.set_cell_as_wall(1, row, column);
                }
            }
        }

        for (row, line) in lines_it.by_ref().enumerate().take(CUBE_SIZE) {
            let mut line_it = line.iter();
            for (column, cell) in line_it.by_ref().skip(CUBE_SIZE).enumerate().take(CUBE_SIZE) {
                if *cell == b'#' {
                    grid.set_cell_as_wall(2, row, column);
                }
            }
        }

        for (row, line) in lines_it.by_ref().enumerate().take(CUBE_SIZE) {
            let mut line_it = line.iter();
            for (column, cell) in line_it.by_ref().enumerate().take(CUBE_SIZE) {
                if *cell == b'#' {
                    grid.set_cell_as_wall(3, row, column);
                }
            }
            for (column, cell) in line_it.by_ref().enumerate().take(CUBE_SIZE) {
                if *cell == b'#' {
                    grid.set_cell_as_wall(4, row, column);
                }
            }
        }

        for (row, line) in lines_it.by_ref().enumerate().take(CUBE_SIZE) {
            let mut line_it = line.iter();
            for (column, cell) in line_it.by_ref().enumerate().take(CUBE_SIZE) {
                if *cell == b'#' {
                    grid.set_cell_as_wall(5, row, column);
                }
            }
        }

        grid
    }
}

fn parse_instructions(input: &[u8]) -> Vec<Instruction> {
    let bytes = input
        .trim_ascii_end()
        .rsplit(|byte| *byte == b'\n')
        .next()
        .unwrap()
        .iter();

    let mut buf: Vec<u8> = Vec::with_capacity(2);
    let mut instructions = Vec::new();

    for byte in bytes {
        match byte {
            b'L' => {
                if !buf.is_empty() {
                    instructions.push(Instruction::Fwd(usize_from_bytes(&buf)));
                    buf.clear()
                }
                instructions.push(Instruction::L)
            }
            b'R' => {
                if !buf.is_empty() {
                    instructions.push(Instruction::Fwd(usize_from_bytes(&buf)));
                    buf.clear()
                }
                instructions.push(Instruction::R)
            }
            _ => buf.push(*byte),
        }
    }

    if !buf.is_empty() {
        instructions.push(Instruction::Fwd(usize_from_bytes(&buf)));
    }

    instructions
}

fn solution(input: &[u8], faces: &[Face]) -> usize {
    let instructions = parse_instructions(input);
    let grid = Grid::parse(input);

    let (face, dir, row, col) = instructions.iter().fold(
        (&faces[0], Direction::R, 0_usize, 0_usize),
        |(face, dir, row, col), instruction| {
            if let Instruction::Fwd(v) = instruction {
                let (mut face, mut dir, mut row, mut col) = (face, dir, row, col);

                for _ in 0..*v {
                    if let AfterStep::Proceed(next_face_id, next_direction, next_row, next_col) =
                        grid.fwd(faces, &dir, face, row, col)
                    {
                        face = &faces[next_face_id];
                        dir = next_direction;
                        row = next_row;
                        col = next_col;
                    } else {
                        break;
                    }
                }

                (face, dir, row, col)
            } else {
                (face, dir.turn(instruction), row, col)
            }
        },
    );

    (face.position.0 + row + 1) * 1000 + 4 * (face.position.1 + col + 1) + dir.score()
}

fn part1(input: &[u8]) -> usize {
    solution(input, &FACES_2D)
}

fn part2(input: &[u8]) -> usize {
    solution(input, &FACES_3D)
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
        assert_eq!(part1(INPUT), 67390)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 95291)
    }

    #[bench]
    fn bench_parse_grid(b: &mut Bencher) {
        b.iter(|| Grid::parse(INPUT))
    }

    #[bench]
    fn bench_parse_instructions(b: &mut Bencher) {
        b.iter(|| parse_instructions(INPUT))
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
