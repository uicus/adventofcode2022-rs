use super::common;

static ALL_MOVES: [common::YourMove; 3] = [
    common::YourMove::X,
    common::YourMove::Y,
    common::YourMove::Z,
];

fn inferred_move(move1: common::Move, result: common::YourMove) -> common::YourMove {
    let expected = common::result_correspondence(result);
    for potential_result in ALL_MOVES {
        if common::round_result(move1, common::your_correspondence(potential_result)) == expected {
            return potential_result;
        }
    }
    common::YourMove::X
}

pub fn solve(input: &Vec<(common::OpponentMove, common::YourMove)>) -> u32 {
    common::count_result(
        &input
            .iter()
            .map(|&(opponent, expected)| {
                (
                    opponent,
                    inferred_move(common::opponent_correspondence(opponent), expected),
                )
            })
            .collect(),
    )
}
