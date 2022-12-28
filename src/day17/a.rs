use super::common;

pub fn solve(input: &Vec<common::Direction>) -> usize {
    let mut current_direction = 0;
    let mut brick_index = 0;
    let mut current_tower = vec![];
    for _ in 0..2022 {
        current_direction =
            common::process_brick(brick_index, current_direction, input, &mut current_tower);
        brick_index = (brick_index + 1) % common::POSSIBLE_BRCIKS.len();
    }
    common::height(&current_tower)
}
