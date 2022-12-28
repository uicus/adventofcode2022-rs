use crate::utils;
use std::fs;

#[derive(Copy, Clone)]
pub enum Command {
    Noop,
    Addx(i32),
}

pub fn change_register(command: Command, previous_value: i32) -> i32 {
    match command {
        Command::Addx(x) => previous_value + x,
        Command::Noop => previous_value,
    }
}

pub fn cycles(command: Command) -> u32 {
    match command {
        Command::Addx(_) => 2,
        Command::Noop => 1,
    }
}

fn parse_line(line: &str) -> Result<Command, Box<dyn std::error::Error>> {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    if parts[0] == "noop" {
        Ok(Command::Noop)
    } else if parts[0] == "addx" {
        if parts.len() == 2 {
            Ok(Command::Addx(parts[1].parse()?))
        } else {
            Err(Box::new(utils::ParseInputError))
        }
    } else {
        Err(Box::new(utils::ParseInputError))
    }
}

pub fn read(filename: &str) -> Result<Vec<Command>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
