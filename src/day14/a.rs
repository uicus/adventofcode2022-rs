use super::common;

pub fn solve(input: &Vec<Vec<(isize, isize)>>) -> u32 {
    let mut map = common::create_map(input);
    while !common::drop_sand(&mut map, (500, 0)) {}
    map.count_sand()
}
