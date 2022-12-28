use crate::utils;
use std::fs;

pub enum Digit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

fn parse_line(line: &str) -> Result<Vec<Digit>, Box<dyn std::error::Error>> {
    line.chars()
        .map(|c| match c {
            '2' => Ok(Digit::Two),
            '1' => Ok(Digit::One),
            '0' => Ok(Digit::Zero),
            '-' => Ok(Digit::Minus),
            '=' => Ok(Digit::DoubleMinus),
            _ => Err(Box::new(utils::ParseInputError) as Box<dyn std::error::Error>),
        })
        .collect()
}

pub fn read(filename: &str) -> Result<Vec<Vec<Digit>>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
