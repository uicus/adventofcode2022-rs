use crate::utils;
use std::collections;
use std::fs;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Operation {
    direction: Direction,
    count: usize,
}

fn should_move_tail(head: (i32, i32), tail: (i32, i32)) -> bool {
    (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1
}

fn move_along_axis(head: i32, tail: i32) -> i32 {
    if head < tail {
        tail - 1
    } else if head > tail {
        tail + 1
    } else {
        tail
    }
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    (
        move_along_axis(head.0, tail.0),
        move_along_axis(head.1, tail.1),
    )
}

fn move_once(head: (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (head.0 - 1, head.1),
        Direction::Down => (head.0 + 1, head.1),
        Direction::Left => (head.0, head.1 - 1),
        Direction::Right => (head.0, head.1 + 1),
    }
}

fn move_tail_if_needed(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if should_move_tail(head, tail) {
        move_tail(head, tail)
    } else {
        tail
    }
}

pub fn move_whole(
    head: (i32, i32),
    tails: &mut Vec<(i32, i32)>,
    operation: Operation,
    visited: &mut collections::HashSet<(i32, i32)>,
) -> (i32, i32) {
    let mut current_head = head;
    for _ in 0..operation.count {
        current_head = move_once(current_head, operation.direction);
        tails[0] = move_tail_if_needed(current_head, tails[0]);
        for index in 1..tails.len() {
            tails[index] = move_tail_if_needed(tails[index - 1], tails[index]);
        }
        visited.insert(*tails.last().unwrap_or(&(0, 0)));
    }
    current_head
}

fn parse_line(line: &str) -> Result<Operation, Box<dyn std::error::Error>> {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        let direction = match parts[0] {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(Box::new(utils::ParseInputError)),
        }?;
        Ok(Operation {
            direction: direction,
            count: parts[1].parse()?,
        })
    }
}

pub fn read(filename: &str) -> Result<Vec<Operation>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
