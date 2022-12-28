use super::common;
use std::collections;

pub fn solve(input: &Vec<common::Direction>) -> u64 {
    let mut current_direction = 0;
    let mut brick_index = 0;
    let mut current_tower = vec![];
    let mut count = 0;
    let mut cache = collections::HashMap::new();
    let mut diffs = collections::HashMap::new();
    loop {
        if let Some((prev_height, prev_count)) = cache.get(&(current_direction, brick_index)) {
            if let None = diffs.get(&(current_direction, brick_index)) {
                diffs.insert((current_direction, brick_index), vec![]);
            }
            if let Some(l) = diffs.get_mut(&(current_direction, brick_index)) {
                l.push((
                    common::height(&current_tower) - prev_height,
                    count - prev_count,
                ));
                if l.len() > 5 {
                    let remaining_cycles =
                        (1000000000000 - count as u64) % (count - prev_count) as u64;
                    let additional_rounds =
                        (1000000000000 - count as u64) / (count - prev_count) as u64;
                    let bricks_per_cycle = common::height(&current_tower) - prev_height;
                    for _ in 0..remaining_cycles {
                        current_direction = common::process_brick(
                            brick_index,
                            current_direction,
                            input,
                            &mut current_tower,
                        );
                        brick_index = (brick_index + 1) % common::POSSIBLE_BRCIKS.len();
                    }
                    return common::height(&current_tower) as u64
                        + additional_rounds * bricks_per_cycle as u64;
                }
            }
        }
        cache.insert(
            (current_direction, brick_index),
            (common::height(&current_tower), count),
        );
        current_direction =
            common::process_brick(brick_index, current_direction, input, &mut current_tower);
        brick_index = (brick_index + 1) % common::POSSIBLE_BRCIKS.len();
        count += 1;
    }
}
