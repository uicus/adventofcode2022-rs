use super::common;

pub fn solve(input: &Vec<common::Monkey>) -> i64 {
    let all_monkeys = input
        .iter()
        .map(|x| (x.name.clone(), x.operation.clone()))
        .collect();
    common::value(&"root".to_string(), &all_monkeys)
}
