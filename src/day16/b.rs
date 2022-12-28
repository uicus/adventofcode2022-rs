use super::common;
use std::collections;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    opened: u64,
    time: u32,
    place1: usize,
    place2: usize,
}

fn reverse(state: State) -> State {
    State {
        place1: state.place2,
        place2: state.place1,
        ..state
    }
}

fn count_valves(
    state: State,
    previous_place1: usize,
    previous_place2: usize,
    result_per_minute: u32,
    results: &mut collections::HashMap<State, u32>,
    graph: &common::Graph,
    result_so_far: i32,
    estimate: &mut i32,
    all_weights: i32,
) -> u32 {
    if result_so_far < *estimate {
        if state.time as i32 * all_weights < *estimate - result_so_far {
            return 0;
        }
    }
    if state.time == 0 {
        return 0;
    }
    if let Some(result) = results.get(&state) {
        return *result;
    }
    if let Some(result) = results.get(&reverse(state)) {
        return *result;
    }
    let result = common::perform_step(
        state.opened,
        state.place1,
        previous_place1,
        result_per_minute,
        graph,
    )
    .iter()
    .flat_map(|&(opened, place1, result_per_minute)| {
        common::perform_step(
            opened,
            state.place2,
            previous_place2,
            result_per_minute,
            graph,
        )
        .iter()
        .map(|&(opened, place2, result_per_minute)| (opened, place1, place2, result_per_minute))
        .collect::<Vec<_>>()
    })
    .map(
        |(opened, next_place1, next_place2, next_result_per_minute)| {
            count_valves(
                State {
                    opened: opened,
                    time: state.time - 1,
                    place1: next_place1,
                    place2: next_place2,
                },
                state.place1,
                state.place2,
                next_result_per_minute,
                results,
                graph,
                result_so_far + result_per_minute as i32,
                estimate,
                all_weights,
            )
        },
    )
    .max()
    .unwrap_or(0)
        + result_per_minute;
    results.insert(state, result);
    if result as i32 + result_so_far > *estimate {
        *estimate = result as i32 + result_so_far;
    }
    result
}

pub fn solve(input: &(common::Graph, usize)) -> u32 {
    let mut results = collections::HashMap::new();
    count_valves(
        State {
            opened: 0,
            time: 26,
            place1: input.1,
            place2: input.1,
        },
        input.1,
        input.1,
        0,
        &mut results,
        &input.0,
        0,
        &mut 0,
        common::sum_weights(&input.0) as i32,
    )
}
