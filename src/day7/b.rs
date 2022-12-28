use super::common;

pub fn solve(input: &Vec<common::Line>) -> u32 {
    let state = common::create_filesystem(input);
    let mut sizes = vec![];
    let space_taken = common::directories_sizes(&state, &mut sizes);
    sizes
        .iter()
        .map(|&(_, x)| x)
        .filter(|&size| size >= space_taken - 40000000)
        .min()
        .unwrap_or(0)
}
