use super::common;
use std::collections;

fn all_alone(elves: &collections::HashSet<(i32, i32)>) -> bool {
    for elf in elves {
        for y in (elf.0 - 1)..=(elf.0 + 1) {
            for x in (elf.1 - 1)..=(elf.1 + 1) {
                if (y != elf.0 || x != elf.1) && elves.contains(&(y, x)) {
                    return false;
                }
            }
        }
    }
    true
}

pub fn solve(input: &Vec<(i32, i32)>) -> i32 {
    let mut directions = common::create_directions();
    let mut elves = input.iter().cloned().collect();
    let mut round = 1;
    while !all_alone(&elves) {
        elves = common::perform_round(&mut directions, elves);
        round += 1;
    }
    round
}
