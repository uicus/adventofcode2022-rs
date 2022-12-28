use super::common;
use std::cmp;

pub fn solve(input: &Vec<(common::Packet, common::Packet)>) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(index, (x, y))| (index, common::compare(x, y)))
        .filter(|&(_, x)| x == cmp::Ordering::Less)
        .map(|(index, _)| (index + 1))
        .sum()
}
