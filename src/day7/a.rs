use super::common;

pub fn solve(input: &Vec<common::Line>) -> u32 {
    let state = common::create_filesystem(input);
    let mut sizes = vec![];
    common::directories_sizes(&state, &mut sizes);
    sizes
        .iter()
        .map(|&(_, x)| x)
        .filter(|&size| size <= 100000)
        .sum()
}
