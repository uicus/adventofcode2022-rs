use crate::utils;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    End,
}

pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
}

pub enum Instruction {
    Forward(i32),
    Left,
    Right,
}

pub fn rotate_left(direction: (i32, i32)) -> (i32, i32) {
    (-direction.1, direction.0)
}

pub fn rotate_right(direction: (i32, i32)) -> (i32, i32) {
    (direction.1, -direction.0)
}

pub fn outside(coords: (i32, i32), grid: &Grid) -> bool {
    coords.0 < 0
        || coords.0 >= grid.cells.len() as i32
        || coords.1 < 0
        || coords.1 >= grid.cells[0].len() as i32
        || grid.cells[coords.0 as usize][coords.1 as usize] == Cell::End
}

pub fn start_coords(grid: &Grid) -> (i32, i32) {
    for (x, cell) in grid.cells[0].iter().enumerate() {
        if cell == &Cell::Empty {
            return (0, x as i32);
        }
    }
    (0, 0)
}

pub fn direction_value(direction: (i32, i32)) -> i32 {
    match direction {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => 999,
    }
}

fn parse_cell(text: char) -> Result<Cell, Box<dyn std::error::Error>> {
    match text {
        ' ' => Ok(Cell::End),
        '.' => Ok(Cell::Empty),
        '#' => Ok(Cell::Wall),
        _ => Err(Box::new(utils::ParseInputError)),
    }
}

fn parse_line(line: &str) -> Result<Vec<Cell>, Box<dyn std::error::Error>> {
    line.chars().map(parse_cell).collect()
}

fn align(grid: &mut Grid) {
    let width = grid.cells.iter().map(|x| x.len()).max().unwrap_or(0);
    for row in grid.cells.iter_mut() {
        row.resize(width, Cell::End);
    }
}

fn parse_grid(text: &str) -> Result<Grid, Box<dyn std::error::Error>> {
    let mut grid = Grid {
        cells: text.lines().map(parse_line).collect::<Result<_, _>>()?,
    };
    align(&mut grid);
    Ok(grid)
}

fn parse_instructions(text: &str) -> Result<Vec<Instruction>, Box<dyn std::error::Error>> {
    let mut result = vec![];
    let mut current_int = 0;
    for next_char in text.chars() {
        if next_char.is_digit(10) {
            current_int *= 10;
            current_int += next_char as i32 - '0' as i32;
        } else {
            if current_int > 0 {
                result.push(Instruction::Forward(current_int));
                current_int = 0;
            }
            match next_char {
                'R' => result.push(Instruction::Right),
                'L' => result.push(Instruction::Left),
                _ => return Err(Box::new(utils::ParseInputError)),
            }
        }
    }
    if current_int > 0 {
        result.push(Instruction::Forward(current_int));
    }
    Ok(result)
}

pub fn read(filename: &str) -> Result<(Grid, Vec<Instruction>), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(filename)?;
    let parts = text.split("\n\n").collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok((parse_grid(parts[0])?, parse_instructions(parts[1].trim())?))
    }
}
