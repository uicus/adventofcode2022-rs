use crate::utils;
use std::fs;

fn parse_line(line: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    line.chars()
        .map(|x| {
            x.to_digit(10)
                .ok_or(Box::new(utils::ParseInputError) as Box<dyn std::error::Error>)
        })
        .collect()
}

pub fn read(filename: &str) -> Result<Vec<Vec<u32>>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
