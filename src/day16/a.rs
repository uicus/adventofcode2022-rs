use super::common;
use std::collections;

#[derive(PartialEq, Eq, Hash)]
struct State {
    opened: u64,
    time: u32,
    place: usize,
}

fn count_valves(
    state: State,
    previous_place: usize,
    result_per_minute: u32,
    results: &mut collections::HashMap<State, u32>,
    graph: &common::Graph,
) -> u32 {
    if let Some(result) = results.get(&state) {
        return *result;
    }
    if state.time == 0 {
        results.insert(state, 0);
        return 0;
    }
    let result = common::perform_step(
        state.opened,
        state.place,
        previous_place,
        result_per_minute,
        graph,
    )
    .iter()
    .map(|&(opened, next_place, result_per_minute)| {
        count_valves(
            State {
                opened: opened,
                time: state.time - 1,
                place: next_place,
            },
            state.place,
            result_per_minute,
            results,
            graph,
        )
    })
    .max()
    .unwrap_or(0)
        + result_per_minute;
    results.insert(state, result);
    result
}

pub fn solve(input: &(common::Graph, usize)) -> u32 {
    let mut results = collections::HashMap::new();
    count_valves(
        State {
            opened: 0,
            time: 30,
            place: input.1,
        },
        input.1,
        0,
        &mut results,
        &input.0,
    )
}
