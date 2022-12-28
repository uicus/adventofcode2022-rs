use crate::utils;
use std::fs;

pub fn priority(item: u8) -> u32 {
    if item >= b'a' && item <= b'z' {
        (item - b'a' + 1).into()
    } else if item >= b'A' && item <= b'Z' {
        (item - b'A' + 27).into()
    } else {
        0
    }
}

fn parse_line(line: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if line.len() % 2 != 0 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok(line.chars().map(|x| x as u8).collect())
    }
}

pub fn read(filename: &str) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
