use crate::utils;
use std::fs;

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

const WIDTH: usize = 7;

pub static POSSIBLE_BRCIKS: &[&[&[bool]]] = &[
    &[&[true, true, true, true]],
    &[
        &[false, true, false],
        &[true, true, true],
        &[false, true, false],
    ],
    &[
        &[true, true, true],
        &[false, false, true],
        &[false, false, true],
    ],
    &[&[true], &[true], &[true], &[true]],
    &[&[true, true], &[true, true]],
];

fn collides(
    brick_index: usize,
    brick_position: (usize, usize),
    current_tower: &Vec<[bool; WIDTH]>,
) -> bool {
    for (y, row) in POSSIBLE_BRCIKS[brick_index].iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value && current_tower[brick_position.0 + y][brick_position.1 + x] {
                return true;
            }
        }
    }
    return false;
}

fn move_if_free(
    brick_index: usize,
    brick_position: (usize, usize),
    new_brick_position: (usize, usize),
    current_tower: &Vec<[bool; WIDTH]>,
) -> (usize, usize) {
    if collides(brick_index, new_brick_position, current_tower) {
        brick_position
    } else {
        new_brick_position
    }
}

fn shift_sideway(
    brick_index: usize,
    brick_position: (usize, usize),
    current_tower: &Vec<[bool; WIDTH]>,
    shift: Direction,
) -> (usize, usize) {
    match shift {
        Direction::Left => {
            if brick_position.1 == 0 {
                brick_position
            } else {
                move_if_free(
                    brick_index,
                    brick_position,
                    (brick_position.0, brick_position.1 - 1),
                    current_tower,
                )
            }
        }
        Direction::Right => {
            if brick_position.1 + POSSIBLE_BRCIKS[brick_index][0].len() == WIDTH {
                brick_position
            } else {
                move_if_free(
                    brick_index,
                    brick_position,
                    (brick_position.0, brick_position.1 + 1),
                    current_tower,
                )
            }
        }
    }
}

fn shift_downwards(
    brick_index: usize,
    brick_position: (usize, usize),
    current_tower: &Vec<[bool; WIDTH]>,
) -> (usize, usize) {
    if brick_position.0 == 0 {
        brick_position
    } else {
        move_if_free(
            brick_index,
            brick_position,
            (brick_position.0 - 1, brick_position.1),
            current_tower,
        )
    }
}

fn soldify(
    brick_index: usize,
    brick_position: (usize, usize),
    current_tower: &mut Vec<[bool; WIDTH]>,
) {
    for (y, row) in POSSIBLE_BRCIKS[brick_index].iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value {
                current_tower[brick_position.0 + y][brick_position.1 + x] = true;
            }
        }
    }
}

pub fn height(current_tower: &Vec<[bool; WIDTH]>) -> usize {
    for y in 0..current_tower.len() {
        if current_tower[current_tower.len() - y - 1]
            .iter()
            .any(|&x| x)
        {
            return current_tower.len() - y;
        }
    }
    return 0;
}

fn put_new_brick(brick_index: usize, current_tower: &mut Vec<[bool; WIDTH]>) -> (usize, usize) {
    let y = height(current_tower) + 3;
    current_tower.resize(
        y + POSSIBLE_BRCIKS[brick_index].len(),
        [false, false, false, false, false, false, false],
    );
    (y, 2)
}

pub fn process_brick(
    brick_index: usize,
    mut current_direction: usize,
    directions: &Vec<Direction>,
    current_tower: &mut Vec<[bool; WIDTH]>,
) -> usize {
    let mut current_position = put_new_brick(brick_index, current_tower);
    let mut changed = true;
    while changed {
        current_position = shift_sideway(
            brick_index,
            current_position,
            current_tower,
            directions[current_direction],
        );
        current_direction = (current_direction + 1) % directions.len();
        let next_position = shift_downwards(brick_index, current_position, current_tower);
        changed = next_position != current_position;
        current_position = next_position;
    }
    soldify(brick_index, current_position, current_tower);
    current_direction
}

fn parse_direction(raw: char) -> Result<Direction, Box<dyn std::error::Error>> {
    match raw {
        '<' => Ok(Direction::Left),
        '>' => Ok(Direction::Right),
        _ => Err(Box::new(utils::ParseInputError)),
    }
}

pub fn read(filename: &str) -> Result<Vec<Direction>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .trim()
        .chars()
        .map(parse_direction)
        .collect()
}
