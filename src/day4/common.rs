use crate::utils;
use std::fs;

fn parse_segment(segment: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let points = segment.split('-').take(2).collect::<Vec<&str>>();
    if points.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok((points[0].parse()?, points[1].parse()?))
    }
}

fn parse_line(line: &str) -> Result<((u32, u32), (u32, u32)), Box<dyn std::error::Error>> {
    let segments = line.split(',').take(2).collect::<Vec<&str>>();
    if segments.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok((parse_segment(segments[0])?, parse_segment(segments[1])?))
    }
}

pub fn read(filename: &str) -> Result<Vec<((u32, u32), (u32, u32))>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
