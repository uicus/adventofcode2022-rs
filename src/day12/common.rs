use crate::utils;
use std::collections;
use std::fs;

#[derive(Copy, Clone)]
pub enum Cell {
    Start,
    End,
    Height(u32),
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

fn create_empty_visit_map(grid: &Grid) -> Vec<Vec<bool>> {
    grid.cells
        .iter()
        .map(|row| vec![false; row.len()])
        .collect()
}

pub fn find_end(grid: &Grid) -> (isize, isize) {
    for (x, row) in grid.cells.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if let Cell::End = cell {
                return (x as isize, y as isize);
            }
        }
    }
    return (0, 0);
}

fn on_the_grid(coords: (isize, isize), grid: &Grid) -> bool {
    coords.0 >= 0
        && (coords.0 as usize) < grid.cells.len()
        && coords.1 >= 0
        && (coords.1 as usize) < grid.cells[0].len()
}

fn height(coords: (isize, isize), grid: &Grid) -> u32 {
    match grid.cells[coords.0 as usize][coords.1 as usize] {
        Cell::Start => 0,
        Cell::End => 'z' as u32 - 'a' as u32,
        Cell::Height(x) => x,
    }
}

fn passable(
    coords_source: (isize, isize),
    coords_destination: (isize, isize),
    grid: &Grid,
) -> bool {
    height(coords_source, grid) + 1 >= height(coords_destination, grid)
}

pub fn traverse(end: (isize, isize), grid: &Grid, found: fn(Cell) -> bool) -> u32 {
    let mut queue = collections::VecDeque::new();
    let mut visited = create_empty_visit_map(grid);
    queue.push_back((end, 0));
    while !queue.is_empty() {
        let ((x, y), distance) = queue.pop_front().unwrap_or((end, 0));
        if found(grid.cells[x as usize][y as usize]) {
            return distance;
        }
        for (new_x, new_y) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if on_the_grid((new_x, new_y), grid)
                && passable((new_x, new_y), (x, y), grid)
                && !visited[new_x as usize][new_y as usize]
            {
                visited[new_x as usize][new_y as usize] = true;
                queue.push_back(((new_x, new_y), distance + 1))
            }
        }
    }
    return 0;
}

fn parse_cell(text: char) -> Result<Cell, Box<dyn std::error::Error>> {
    if text >= 'a' && text <= 'z' {
        Ok(Cell::Height(text as u32 - 'a' as u32))
    } else if text == 'S' {
        Ok(Cell::Start)
    } else if text == 'E' {
        Ok(Cell::End)
    } else {
        Err(Box::new(utils::ParseInputError))
    }
}

fn parse_line(line: &str) -> Result<Vec<Cell>, Box<dyn std::error::Error>> {
    line.chars().map(parse_cell).collect()
}

pub fn read(filename: &str) -> Result<Grid, Box<dyn std::error::Error>> {
    Ok(Grid {
        cells: fs::read_to_string(filename)?
            .lines()
            .map(parse_line)
            .collect::<Result<_, _>>()?,
    })
}
