use super::common;

pub fn solve(input: &Vec<common::Blueprint>) -> i32 {
    input
        .iter()
        .take(3)
        .map(|blueprint| {
            common::go(
                &common::State {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                    remaining_time: 32,
                },
                blueprint,
                &mut 0,
            )
        })
        .fold(1, |x, y| x * y)
}
