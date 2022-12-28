use super::common;

pub fn solve(input: &Vec<(common::Packet, common::Packet)>) -> usize {
    let two = common::Packet::List(vec![common::Packet::List(vec![common::Packet::Integer(2)])]);
    let six = common::Packet::List(vec![common::Packet::List(vec![common::Packet::Integer(6)])]);
    let mut to_sort = input
        .iter()
        .map(|(x, _)| (x.clone(), false))
        .collect::<Vec<_>>();
    to_sort.extend(input.iter().map(|(_, y)| (y.clone(), false)));
    to_sort.push((two, true));
    to_sort.push((six, true));
    to_sort.sort_by(|(a, _), (b, _)| common::compare(a, b));
    to_sort
        .iter()
        .enumerate()
        .filter(|(_, (_, x))| *x)
        .map(|(x, _)| x + 1)
        .fold(1, |x, y| x * y)
}
