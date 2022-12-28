use crate::utils;
use std::fs;

struct Cost {
    ore: i32,
    clay: i32,
    obsidian: i32,
}

pub struct Blueprint {
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct State {
    pub ore: i32,
    pub clay: i32,
    pub obsidian: i32,
    pub geode: i32,
    pub ore_robots: i32,
    pub clay_robots: i32,
    pub obsidian_robots: i32,
    pub geode_robots: i32,
    pub remaining_time: u32,
}

fn ceiling(a: i32, b: i32) -> i32 {
    if a < 1 {
        0
    } else {
        std::cmp::max((a - 1) / b + 1, 0)
    }
}

fn turns_per_resource(stockpile: i32, cost: i32, robots: i32) -> i32 {
    if robots > 0 {
        ceiling(cost - stockpile, robots)
    } else if cost > 0 {
        1000
    } else {
        0
    }
}

fn turn_to_complete(state: &State, cost: &Cost) -> i32 {
    let ore_turns = turns_per_resource(state.ore, cost.ore, state.ore_robots);
    let clay_turns = turns_per_resource(state.clay, cost.clay, state.clay_robots);
    let obsidian_turns = turns_per_resource(state.obsidian, cost.obsidian, state.obsidian_robots);
    std::cmp::max(std::cmp::max(ore_turns, clay_turns), obsidian_turns) + 1
}

fn wait_x_turns(state: &State, turns: i32) -> State {
    State {
        ore: state.ore + turns * state.ore_robots,
        clay: state.clay + turns * state.clay_robots,
        obsidian: state.obsidian + turns * state.obsidian_robots,
        geode: state.geode + turns * state.geode_robots,
        remaining_time: state.remaining_time - turns as u32,
        ..*state
    }
}

fn suffer_cost(state: &State, cost: &Cost) -> State {
    State {
        ore: state.ore - cost.ore,
        clay: state.clay - cost.clay,
        obsidian: state.obsidian - cost.obsidian,
        ..*state
    }
}

fn possible_steps(state: &State, blueprint: &Blueprint) -> Vec<State> {
    let mut result = vec![];
    let turns = turn_to_complete(state, &blueprint.ore_robot);
    if turns <= state.remaining_time as i32 {
        result.push(State {
            ore_robots: state.ore_robots + 1,
            ..suffer_cost(&wait_x_turns(state, turns), &blueprint.ore_robot)
        })
    }
    let turns = turn_to_complete(state, &blueprint.clay_robot);
    if turns <= state.remaining_time as i32 {
        result.push(State {
            clay_robots: state.clay_robots + 1,
            ..suffer_cost(&wait_x_turns(state, turns), &blueprint.clay_robot)
        })
    }
    let turns = turn_to_complete(state, &blueprint.obsidian_robot);
    if turns <= state.remaining_time as i32 {
        result.push(State {
            obsidian_robots: state.obsidian_robots + 1,
            ..suffer_cost(&wait_x_turns(state, turns), &blueprint.obsidian_robot)
        })
    }
    let turns = turn_to_complete(state, &blueprint.geode_robot);
    if turns <= state.remaining_time as i32 {
        result.push(State {
            geode_robots: state.geode_robots + 1,
            ..suffer_cost(&wait_x_turns(state, turns), &blueprint.geode_robot)
        })
    }
    result
}

pub fn go(state: &State, blueprint: &Blueprint, previous_best: &mut i32) -> i32 {
    if state.geode
        + state.geode_robots * state.remaining_time as i32
        + ((state.remaining_time * (state.remaining_time - 1)) / 2) as i32
        <= *previous_best
    {
        return 0;
    }
    let result = possible_steps(state, blueprint)
        .iter()
        .map(|x| go(x, blueprint, previous_best))
        .max()
        .unwrap_or(state.geode + state.geode_robots * state.remaining_time as i32);
    if result > *previous_best {
        *previous_best = result;
    }
    result
}

fn parse_cost(text: &str) -> Result<Cost, Box<dyn std::error::Error>> {
    let parts = text.split_whitespace().collect::<Vec<_>>();
    let mut result = Cost {
        ore: 0,
        clay: 0,
        obsidian: 0,
    };
    if parts.len() % 3 != 0 {
        return Err(Box::new(utils::ParseInputError));
    }
    for i in 0..parts.len() / 3 {
        match parts[i * 3 + 2] {
            "ore" => result.ore = parts[i * 3 + 1].parse()?,
            "clay" => result.clay = parts[i * 3 + 1].parse()?,
            "obsidian" => result.obsidian = parts[i * 3 + 1].parse()?,
            _ => return Err(Box::new(utils::ParseInputError)),
        }
    }
    Ok(result)
}

fn parse_line(line: &str) -> Result<Blueprint, Box<dyn std::error::Error>> {
    let parts = line.split(".").collect::<Vec<_>>();
    if parts.len() != 4 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok(Blueprint {
            ore_robot: parse_cost(parts[0])?,
            clay_robot: parse_cost(parts[1])?,
            obsidian_robot: parse_cost(parts[2])?,
            geode_robot: parse_cost(parts[3])?,
        })
    }
}

pub fn read(filename: &str) -> Result<Vec<Blueprint>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
