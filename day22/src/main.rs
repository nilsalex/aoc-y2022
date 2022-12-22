#![feature(byte_slice_trim_ascii)]

const INPUT: &[u8] = include_bytes!("input.txt");

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
    D
}

impl Direction {
    fn turn(&self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Fwd(_) => *self,
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
struct Face {
    id: usize,
    position: (usize, usize),
    next_up: (u8, Direction),
    next_down: (u8, Direction),
    next_left: (u8, Direction),
    next_right: (u8, Direction),
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Outside,
    Open,
    Wall,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn rows(&self) -> usize {
        self.cells.len()
    }

    fn cols(&self) -> usize {
        self.cells[0].len()
    }

    fn cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    fn fwd_3d(&self, dir: &Direction, face: &Face, row: usize, col: usize) -> (bool, usize, Direction, usize, usize) {
        match dir {
            Direction::U => {}
            Direction::L => {}
            Direction::R => {}
            Direction::D => {}
        }
        panic!();
    }

    fn fwd(&self, dir: &Direction, row: usize, col: usize) -> (bool, usize, usize) {
        match dir {
            Direction::U => {
                let next_row = row.checked_sub(1).unwrap_or(self.rows()-1);

                match self.cell(next_row, col) {
                    Cell::Open => {
                        return (true, next_row, col);
                    },
                    Cell::Wall => {
                        return (false, 0, 0);
                    },
                    Cell::Outside => {
                        return self.fwd(dir, next_row, col);
                    }
                }
            },
            Direction::L => {
                let next_col = col.checked_sub(1).unwrap_or(self.cols()-1);

                match self.cell(row, next_col) {
                    Cell::Open => {
                        return (true, row, next_col);
                    },
                    Cell::Wall => {
                        return (false, 0, 0);
                    },
                    Cell::Outside => {
                        return self.fwd(dir, row, next_col);
                    }
                }
            },
            Direction::R => {
                let next_col = (col + 1) % self.cols();

                match self.cell(row, next_col) {
                    Cell::Open => {
                        return (true, row, next_col);
                    },
                    Cell::Wall => {
                        return (false, 0, 0);
                    },
                    Cell::Outside => {
                        return self.fwd(dir, row, next_col);
                    }
                }
            }
            Direction::D => {
                let next_row = (row + 1) % self.rows();

                match self.cell(next_row, col) {
                    Cell::Open => {
                        return (true, next_row, col);
                    },
                    Cell::Wall => {
                        return (false, 0, 0);
                    },
                    Cell::Outside => {
                        return self.fwd(dir, next_row, col);
                    }
                }
            }
        }
    }
}

fn part1(input: &[u8]) -> usize {
    let input = input.trim_ascii_end();

    let input_instructions = input.split(|byte| *byte == b'\n').last().unwrap();

    let instructions = input_instructions
        .split_inclusive(|byte| *byte == b'L' || *byte == b'R')
        .flat_map(|str| {
            if let Ok(v) = std::str::from_utf8(str).unwrap().parse::<usize>() {
                vec![Instruction::Fwd(v)]
            } else {
                vec![Instruction::Fwd(std::str::from_utf8(&str[0..str.len() - 1]).unwrap().parse::<usize>().unwrap()),
                     match str.last().unwrap() {
                         b'L' => Instruction::L,
                         b'R' => Instruction::R,
                         _ => panic!(),
                     }]
            }
        })
        .collect::<Vec<Instruction>>();

    let row_size = input.iter().take_while(|byte| **byte != b'\n').count();

    let grid: Grid = Grid {
        cells: input
            .split(|byte| *byte == b'\n')
            .filter(|line| line.len() <= row_size && !line.is_empty())
            .map(|line| line.iter().map(|byte| match byte {
                b' ' => Cell::Outside,
                b'.' => Cell::Open,
                b'#' => Cell::Wall,
                _ => panic!(),
            }).collect::<Vec<Cell>>())
            .map(|mut row| {
                for _ in 0..row_size - row.len() {
                    row.push(Cell::Outside);
                };
                row
            })
            .collect()
    };

    let start_col = grid.cells[0].iter().position(|c| matches!(c, Cell::Open)).unwrap();

    let (mut row, mut col) = (0_usize, start_col);
    let mut dir = Direction::R;

    for instruction in instructions {
        if let Instruction::Fwd(v) = instruction {
            for _ in 0..v {
                let (no_wall, next_row, next_col) = grid.fwd(&dir, row, col);
                if !no_wall {
                    break;
                }
                row = next_row;
                col = next_col;
            }
        } else {
            dir = dir.turn(&instruction)
        }
    }

    (row + 1) * 1000 + 4 * (col + 1) + dir.score()
}

fn part2(input: &[u8]) -> usize {
    let input = input.trim_ascii_end();

    let input_instructions = input.split(|byte| *byte == b'\n').last().unwrap();

    let instructions = input_instructions
        .split_inclusive(|byte| *byte == b'L' || *byte == b'R')
        .flat_map(|str| {
            if let Ok(v) = std::str::from_utf8(str).unwrap().parse::<usize>() {
                vec![Instruction::Fwd(v)]
            } else {
                vec![Instruction::Fwd(std::str::from_utf8(&str[0..str.len() - 1]).unwrap().parse::<usize>().unwrap()),
                     match str.last().unwrap() {
                         b'L' => Instruction::L,
                         b'R' => Instruction::R,
                         _ => panic!(),
                     }]
            }
        })
        .collect::<Vec<Instruction>>();

    let faces = vec![
        Face {
            id: 0,
            position: (0, 50),
            next_up: (5, Direction::R),
            next_down: (2, Direction::D),
            next_left: (3, Direction::R),
            next_right: (1, Direction::R),
        },
        Face {
            id: 1,
            position: (0, 100),
            next_up: (5, Direction::U),
            next_down: (2, Direction::L),
            next_left: (0, Direction::L),
            next_right: (4, Direction::L),
        },
        Face {
            id: 2,
            position: (50, 50),
            next_up: (0, Direction::U),
            next_down: (4, Direction::D),
            next_left: (3, Direction::D),
            next_right: (1, Direction::U),
        },
        Face {
            id: 3,
            position: (100, 0),
            next_up: (2, Direction::R),
            next_down: (5, Direction::D),
            next_left: (0, Direction::R),
            next_right: (4, Direction::R),
        },
        Face {
            id: 4,
            position: (100, 50),
            next_up: (2, Direction::U),
            next_down: (5, Direction::L),
            next_left: (3, Direction::L),
            next_right: (1, Direction::L),
        },
        Face {
            id: 5,
            position: (150, 0),
            next_up: (3, Direction::U),
            next_down: (1, Direction::D),
            next_left: (0, Direction::D),
            next_right: (4, Direction::U),
        },
    ];

    let row_size = input.iter().take_while(|byte| **byte != b'\n').count();

    let grid: Grid = Grid {
        cells: input
            .split(|byte| *byte == b'\n')
            .filter(|line| line.len() <= row_size && !line.is_empty())
            .map(|line| line.iter().map(|byte| match byte {
                b' ' => Cell::Outside,
                b'.' => Cell::Open,
                b'#' => Cell::Wall,
                _ => panic!(),
            }).collect::<Vec<Cell>>())
            .map(|mut row| {
                for _ in 0..row_size - row.len() {
                    row.push(Cell::Outside);
                };
                row
            })
            .collect()
    };

    let mut face: &Face = &faces[0];
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut dir = Direction::R;

    for instruction in instructions {
        if let Instruction::Fwd(v) = instruction {
            for _ in 0..v {
                let (no_wall, next_face_id, next_direction, next_row, next_col) = grid.fwd_3d(&dir, face, row, col);
                if !no_wall {
                    break;
                }
                face = &faces[next_face_id];
                dir = next_direction;
                row = next_row;
                col = next_col;
            }
        } else {
            dir = dir.turn(&instruction)
        }
    }

    (face.position.0 + row + 1) * 1000 + 4 * (face.position.1 + col + 1) + dir.score()
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
