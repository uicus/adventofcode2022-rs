use super::common;

pub fn solve(input: &Vec<i32>) -> i64 {
    let mut current_order = input
        .iter()
        .cloned()
        .map(|x| x as i64 * 811589153)
        .enumerate()
        .collect::<Vec<_>>();
    for _ in 0..10 {
        for i in 0..input.len() {
            common::shift(common::find_position(i, &current_order), &mut current_order);
        }
    }
    let zero_position = common::find_zero(&current_order);
    current_order[(zero_position + 1000) % current_order.len()].1
        + current_order[(zero_position + 2000) % current_order.len()].1
        + current_order[(zero_position + 3000) % current_order.len()].1
}
