use super::common;
use std::collections;

pub fn solve(input: &(common::Grid, Vec<((i32, i32), (i32, i32))>)) -> u32 {
    let mut positions = collections::HashSet::new();
    positions.insert(common::start_position(&input.0));
    let mut blizzards = input.1.clone();
    let mut count = 0;
    while !positions.contains(&common::end_position(&input.0)) {
        (blizzards, positions) = common::perform_round(blizzards, positions, &input.0);
        count += 1;
    }
    count
}
