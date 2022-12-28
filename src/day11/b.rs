use super::common;

pub fn solve(input: &Vec<(common::Monkey, Vec<u32>)>) -> u64 {
    let monkeys = input.iter().map(|(x, _)| x).cloned().collect::<Vec<_>>();
    let mut items = input.iter().map(|(_, x)| x).cloned().collect::<Vec<_>>();
    let mut inspections = vec![0; monkeys.len()];
    let lowest_common_multiply = common::calculate_common_multiple(&monkeys);
    for _ in 0..10000 {
        common::take_round(
            &monkeys,
            &mut items,
            &mut inspections,
            1,
            lowest_common_multiply,
        );
    }
    inspections.sort_unstable();
    inspections
        .iter()
        .rev()
        .take(2)
        .fold(1, |x, y| x as u64 * *y as u64)
}
