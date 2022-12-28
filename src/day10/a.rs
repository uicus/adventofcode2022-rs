use super::common;

pub fn solve(input: &Vec<common::Command>) -> i32 {
    let checkpoints = vec![20, 60, 100, 140, 180, 220];
    let mut current_checkpoint = 0;
    let mut result = 0;
    let mut register = 1;
    let mut cycle = 1;
    for line in input {
        for _ in 0..common::cycles(*line) {
            if current_checkpoint < checkpoints.len() && checkpoints[current_checkpoint] == cycle {
                result += checkpoints[current_checkpoint] as i32 * register;
                current_checkpoint += 1;
            }
            cycle += 1;
        }
        register = common::change_register(*line, register);
    }
    result
}
