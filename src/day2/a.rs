use super::common;

pub fn solve(input: &Vec<(common::OpponentMove, common::YourMove)>) -> u32 {
    common::count_result(input)
}
