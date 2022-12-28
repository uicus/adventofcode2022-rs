use crate::utils;
use std::fs;

pub enum EdgeKind {
    Opening,
    Closing,
}

pub fn crossings(square: ((i32, i32), u32), line: i32) -> Option<(i32, i32)> {
    let (center, radius) = square;
    let distance_from_line = (center.1 - line).abs();
    if distance_from_line <= radius as i32 {
        let remaining_radius = radius as i32 - distance_from_line;
        Some((center.0 - remaining_radius, center.0 + remaining_radius + 1))
    } else {
        None
    }
}

fn distance(point1: (i32, i32), point2: (i32, i32)) -> u32 {
    (point1.0 - point2.0).abs() as u32 + (point1.1 - point2.1).abs() as u32
}

fn parse_point(text: &str) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let parts = text.split(",").collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok((parts[0].parse()?, parts[1].parse()?))
    }
}

fn parse_line(line: &str) -> Result<((i32, i32), u32), Box<dyn std::error::Error>> {
    let parts = line.split(":").collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        let sensor = parse_point(parts[0])?;
        let beacon = parse_point(parts[1])?;
        Ok((sensor, distance(sensor, beacon)))
    }
}

pub fn read(filename: &str) -> Result<Vec<((i32, i32), u32)>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
