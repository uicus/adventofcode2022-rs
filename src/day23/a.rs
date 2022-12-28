use super::common;

pub fn solve(input: &Vec<(i32, i32)>) -> i32 {
    let mut directions = common::create_directions();
    let mut elves = input.iter().cloned().collect();
    for _ in 0..10 {
        elves = common::perform_round(&mut directions, elves);
    }
    let min_y = elves.iter().map(|(y, _)| y).min().unwrap_or(&0);
    let max_y = elves.iter().map(|(y, _)| y).max().unwrap_or(&0);
    let min_x = elves.iter().map(|(_, x)| x).min().unwrap_or(&0);
    let max_x = elves.iter().map(|(_, x)| x).max().unwrap_or(&0);
    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32
}
