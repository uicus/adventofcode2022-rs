use super::common;

pub fn solve(input: &Vec<common::Blueprint>) -> i32 {
    input
        .iter()
        .enumerate()
        .map(|(index, blueprint)| {
            (index as i32 + 1)
                * common::go(
                    &common::State {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                        ore_robots: 1,
                        clay_robots: 0,
                        obsidian_robots: 0,
                        geode_robots: 0,
                        remaining_time: 24,
                    },
                    blueprint,
                    &mut 0,
                )
        })
        .sum()
}
