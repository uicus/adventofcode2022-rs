use super::common;
use std::collections::HashSet;

fn common_item(part1: &Vec<u8>, part2: &Vec<u8>, part3: &Vec<u8>) -> u8 {
    let part1_set: HashSet<u8> = part1.iter().cloned().collect();
    let part2_set: HashSet<u8> = part2.iter().cloned().collect();
    let part3_set: HashSet<u8> = part3.iter().cloned().collect();
    part1_set
        .intersection(&part2_set)
        .cloned()
        .collect::<HashSet<u8>>()
        .intersection(&part3_set)
        .take(1)
        .cloned()
        .collect::<Vec<u8>>()[0]
}

pub fn solve(input: &Vec<Vec<u8>>) -> u32 {
    input[..]
        .chunks(3)
        .map(|x| (&x[0], &x[1], &x[2]))
        .map(|(x, y, z)| common_item(x, y, z))
        .map(common::priority)
        .sum()
}
