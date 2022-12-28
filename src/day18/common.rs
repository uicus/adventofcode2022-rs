use crate::utils;
use std::fs;

fn distance(point1: (i32, i32, i32), point2: (i32, i32, i32)) -> i32 {
    (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs() + (point1.2 - point2.2).abs()
}

pub fn count_outside(cubes: &Vec<(i32, i32, i32)>) -> usize {
    cubes
        .iter()
        .map(|&x| 6 - cubes.iter().filter(|&y| distance(x, *y) == 1).count())
        .sum()
}

fn parse_line(line: &str) -> Result<(i32, i32, i32), Box<dyn std::error::Error>> {
    let parts = line.split(",").collect::<Vec<_>>();
    if parts.len() != 3 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok((parts[0].parse()?, parts[1].parse()?, parts[2].parse()?))
    }
}

pub fn read(filename: &str) -> Result<Vec<(i32, i32, i32)>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
