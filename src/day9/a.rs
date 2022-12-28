use super::common;
use std::collections;

pub fn solve(input: &Vec<common::Operation>) -> usize {
    let mut visited = collections::HashSet::new();
    let mut head = (0, 0);
    let mut tails = vec![(0, 0); 1];
    visited.insert(*tails.last().unwrap_or(&(0, 0)));
    for &operation in input {
        head = common::move_whole(head, &mut tails, operation, &mut visited);
    }
    visited.len()
}
