use super::common;

pub fn solve(input: &Vec<Vec<(isize, isize)>>) -> u32 {
    let mut real_input = input.clone();
    let lowermost = input
        .iter()
        .map(|l| l.iter().map(|(_, y)| y).max().unwrap_or(&0))
        .max()
        .unwrap_or(&0);
    real_input.push(vec![(0, lowermost + 2), (1000, lowermost + 2)]);
    let mut map = common::create_map(&real_input);
    while !common::drop_sand(&mut map, (500, 0)) {}
    map.count_sand()
}
