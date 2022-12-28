use super::common;
use std::collections::HashSet;

fn split_line(line: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let slice_point = line.len() / 2;
    let mut part1 = line.clone();
    let part2 = part1.split_off(slice_point);
    (part1, part2)
}

fn common_item(part1: &Vec<u8>, part2: &Vec<u8>) -> u8 {
    let part1_set: HashSet<u8> = part1.iter().cloned().collect();
    let part2_set: HashSet<u8> = part2.iter().cloned().collect();
    part1_set
        .intersection(&part2_set)
        .take(1)
        .cloned()
        .collect::<Vec<u8>>()[0]
}

pub fn solve(input: &Vec<Vec<u8>>) -> u32 {
    input
        .iter()
        .map(split_line)
        .map(|(x, y)| common_item(&x, &y))
        .map(common::priority)
        .sum()
}
