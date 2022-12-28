use super::common;

fn start_at_edge(coord: i32, direction: i32, edge: usize) -> i32 {
    match direction {
        -1 => edge as i32 - 1,
        0 => coord,
        1 => 0,
        _ => 999,
    }
}

fn move_once_forward(coords: (i32, i32), direction: (i32, i32), grid: &common::Grid) -> (i32, i32) {
    let new_coords = (coords.0 + direction.0, coords.1 + direction.1);
    if common::outside(new_coords, grid) {
        let mut current_coords = (
            start_at_edge(new_coords.0, direction.0, grid.cells.len()),
            start_at_edge(new_coords.1, direction.1, grid.cells[0].len()),
        );
        while common::outside(current_coords, grid) {
            current_coords = (
                current_coords.0 + direction.0,
                current_coords.1 + direction.1,
            );
        }
        current_coords
    } else {
        new_coords
    }
}

fn move_forward(
    mut coords: (i32, i32),
    count: i32,
    direction: (i32, i32),
    grid: &common::Grid,
) -> (i32, i32) {
    for _ in 0..count {
        let new_coords = move_once_forward(coords, direction, grid);
        if grid.cells[new_coords.0 as usize][new_coords.1 as usize] == common::Cell::Wall {
            return coords;
        }
        coords = new_coords;
    }
    coords
}

pub fn solve(input: &(common::Grid, Vec<common::Instruction>)) -> i32 {
    let mut current_coords = common::start_coords(&input.0);
    let mut current_direction = (0, 1);
    for instruction in &input.1 {
        match instruction {
            &common::Instruction::Forward(x) => {
                current_coords = move_forward(current_coords, x, current_direction, &input.0)
            }
            common::Instruction::Left => current_direction = common::rotate_left(current_direction),
            common::Instruction::Right => {
                current_direction = common::rotate_right(current_direction)
            }
        }
    }
    1000 * (current_coords.0 + 1)
        + 4 * (current_coords.1 + 1)
        + common::direction_value(current_direction)
}
