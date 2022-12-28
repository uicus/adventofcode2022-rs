use crate::utils;
use std::collections;
use std::fs;

pub struct Grid {
    height: usize,
    width: usize,
    upper_opening: usize,
    lower_opening: usize,
}

pub fn start_position(grid: &Grid) -> (i32, i32) {
    (0, grid.upper_opening as i32)
}

pub fn end_position(grid: &Grid) -> (i32, i32) {
    (grid.height as i32 - 1, grid.lower_opening as i32)
}

fn possible_moves(
    position: (i32, i32),
    grid: &Grid,
    blizzards: &collections::HashSet<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for (y, x) in [
        position,
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
    ] {
        if (y > 0 || y == 0 && x == grid.upper_opening as i32)
            && (y < grid.height as i32 - 1
                || y == grid.height as i32 - 1 && x == grid.lower_opening as i32)
            && x > 0
            && x < grid.width as i32 - 1
            && !blizzards.contains(&(y, x))
        {
            result.push((y, x));
        }
    }
    result
}

fn next_positions(
    positions: collections::HashSet<(i32, i32)>,
    grid: &Grid,
    blizzards: &collections::HashSet<(i32, i32)>,
) -> collections::HashSet<(i32, i32)> {
    let mut result = collections::HashSet::new();
    for position in positions {
        for new_position in possible_moves(position, grid, blizzards) {
            result.insert(new_position);
        }
    }
    result
}

fn wrap_position(position: (i32, i32), grid: &Grid) -> (i32, i32) {
    if position.0 <= 0 {
        (grid.height as i32 - 2, position.1)
    } else if position.0 >= grid.height as i32 - 1 {
        (1, position.1)
    } else if position.1 <= 0 {
        (position.0, grid.width as i32 - 2)
    } else if position.1 >= grid.width as i32 - 1 {
        (position.0, 1)
    } else {
        position
    }
}

fn move_blizzards(
    blizzards: Vec<((i32, i32), (i32, i32))>,
    grid: &Grid,
) -> Vec<((i32, i32), (i32, i32))> {
    let mut result = vec![];
    for (position, direction) in blizzards {
        result.push((
            wrap_position((position.0 + direction.0, position.1 + direction.1), grid),
            direction,
        ));
    }
    result
}

pub fn perform_round(
    blizzards: Vec<((i32, i32), (i32, i32))>,
    positions: collections::HashSet<(i32, i32)>,
    grid: &Grid,
) -> (
    Vec<((i32, i32), (i32, i32))>,
    collections::HashSet<(i32, i32)>,
) {
    let new_blizzards = move_blizzards(blizzards, grid);
    let blizzards_set = new_blizzards.iter().map(|&(x, _)| x).collect();
    let new_positions = next_positions(positions, grid, &blizzards_set);
    (new_blizzards, new_positions)
}

fn parse_opening_line(line: &str) -> usize {
    for (index, cell) in line.chars().enumerate() {
        if cell == '.' {
            return index;
        }
    }
    0
}

fn parse_middle_line(
    line: &str,
    y: usize,
    points_so_far: &mut Vec<((i32, i32), (i32, i32))>,
) -> Result<(), Box<dyn std::error::Error>> {
    for (x, cell) in line.chars().enumerate() {
        if cell != '#' && cell != '.' {
            let direction = match cell {
                '^' => (-1, 0),
                'v' => (1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                _ => return Err(Box::new(utils::ParseInputError)),
            };
            points_so_far.push(((y as i32, x as i32), direction));
        }
    }
    Ok(())
}

pub fn read(
    filename: &str,
) -> Result<(Grid, Vec<((i32, i32), (i32, i32))>), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(filename)?;
    let lines = text.lines().collect::<Vec<_>>();
    let grid = Grid {
        height: lines.len(),
        width: lines[0].len(),
        upper_opening: parse_opening_line(lines[0]),
        lower_opening: parse_opening_line(lines[lines.len() - 1]),
    };
    let mut points = vec![];
    for (y, line) in lines[1..lines.len() - 1].iter().enumerate() {
        parse_middle_line(line, y + 1, &mut points)?;
    }
    Ok((grid, points))
}
