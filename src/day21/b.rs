use super::common;
use std::collections;

fn list_unknown(
    name: &String,
    all_monkeys: &collections::HashMap<String, common::Operation>,
    results_so_far: &mut collections::HashSet<String>,
) -> bool {
    if name == "humn" {
        results_so_far.insert(name.clone());
        return true;
    }
    if let Some(operation) = all_monkeys.get(name) {
        let result = match operation {
            common::Operation::Const(_) => false,
            common::Operation::Add(x, y) => {
                list_unknown(&x, all_monkeys, results_so_far)
                    || list_unknown(&y, all_monkeys, results_so_far)
            }
            common::Operation::Subtract(x, y) => {
                list_unknown(&x, all_monkeys, results_so_far)
                    || list_unknown(&y, all_monkeys, results_so_far)
            }
            common::Operation::Multiply(x, y) => {
                list_unknown(&x, all_monkeys, results_so_far)
                    || list_unknown(&y, all_monkeys, results_so_far)
            }
            common::Operation::Divide(x, y) => {
                list_unknown(&x, all_monkeys, results_so_far)
                    || list_unknown(&y, all_monkeys, results_so_far)
            }
        };
        if result {
            results_so_far.insert(name.clone());
        }
        return result;
    }
    false
}

fn predict_value(
    name: &String,
    all_monkeys: &collections::HashMap<String, common::Operation>,
    to_predict: &collections::HashSet<String>,
    to_evaluate: i64,
) -> i64 {
    if let Some(operation) = all_monkeys.get(name) {
        match operation {
            common::Operation::Const(_) => {
                if name == "humn" {
                    to_evaluate
                } else {
                    0
                }
            }
            common::Operation::Add(x, y) => {
                if to_predict.contains(x) {
                    predict_value(
                        x,
                        all_monkeys,
                        to_predict,
                        to_evaluate - common::value(y, all_monkeys),
                    )
                } else {
                    predict_value(
                        y,
                        all_monkeys,
                        to_predict,
                        to_evaluate - common::value(x, all_monkeys),
                    )
                }
            }
            common::Operation::Subtract(x, y) => {
                if to_predict.contains(x) {
                    predict_value(
                        x,
                        all_monkeys,
                        to_predict,
                        to_evaluate + common::value(y, all_monkeys),
                    )
                } else {
                    predict_value(
                        y,
                        all_monkeys,
                        to_predict,
                        common::value(x, all_monkeys) - to_evaluate,
                    )
                }
            }
            common::Operation::Multiply(x, y) => {
                if to_predict.contains(x) {
                    predict_value(
                        x,
                        all_monkeys,
                        to_predict,
                        to_evaluate / common::value(y, all_monkeys),
                    )
                } else {
                    predict_value(
                        y,
                        all_monkeys,
                        to_predict,
                        to_evaluate / common::value(x, all_monkeys),
                    )
                }
            }
            common::Operation::Divide(x, y) => {
                if to_predict.contains(x) {
                    predict_value(
                        x,
                        all_monkeys,
                        to_predict,
                        to_evaluate * common::value(y, all_monkeys),
                    )
                } else {
                    predict_value(
                        y,
                        all_monkeys,
                        to_predict,
                        common::value(x, all_monkeys) / to_evaluate,
                    )
                }
            }
        }
    } else {
        0
    }
}

pub fn solve(input: &Vec<common::Monkey>) -> i64 {
    let mut all_monkeys: collections::HashMap<_, _> = input
        .iter()
        .map(|x| (x.name.clone(), x.operation.clone()))
        .collect();
    if let Some(operation) = all_monkeys.get(&"root".to_string()) {
        match operation {
            common::Operation::Add(x, y) => {
                all_monkeys.insert(
                    "root".to_string(),
                    common::Operation::Subtract(x.clone(), y.clone()),
                );
            }
            common::Operation::Subtract(x, y) => {
                all_monkeys.insert(
                    "root".to_string(),
                    common::Operation::Subtract(x.clone(), y.clone()),
                );
            }
            common::Operation::Multiply(x, y) => {
                all_monkeys.insert(
                    "root".to_string(),
                    common::Operation::Subtract(x.clone(), y.clone()),
                );
            }
            common::Operation::Divide(x, y) => {
                all_monkeys.insert(
                    "root".to_string(),
                    common::Operation::Subtract(x.clone(), y.clone()),
                );
            }
            _ => {}
        }
    }
    let mut to_predict = collections::HashSet::new();
    list_unknown(&"root".to_string(), &all_monkeys, &mut to_predict);
    predict_value(&"root".to_string(), &all_monkeys, &to_predict, 0)
}
