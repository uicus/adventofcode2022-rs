use itertools::Itertools;
use std::fs;

fn all_different(chars: &[char]) -> bool {
    chars.iter().unique().count() == chars.len()
}

pub fn find_marker(input: &Vec<char>, size: usize) -> usize {
    for i in 0..=input.len() - size {
        if all_different(&input[i..i + size]) {
            return i + size;
        }
    }
    0
}

pub fn read(filename: &str) -> Result<Vec<char>, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(filename)?.chars().collect())
}
