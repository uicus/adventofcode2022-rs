use crate::utils;
use std::fs;

#[derive(Copy, Clone)]
enum Variable {
    Old,
    Const(u32),
}

#[derive(Copy, Clone)]
enum Operation {
    Add(Variable, Variable),
    Multiple(Variable, Variable),
}

#[derive(Copy, Clone)]
struct Test {
    divisor: u32,
    if_true: usize,
    if_false: usize,
}

#[derive(Copy, Clone)]
pub struct Monkey {
    operation: Operation,
    test: Test,
}

fn retrieve_value(item: u32, variable: Variable) -> u32 {
    match variable {
        Variable::Old => item,
        Variable::Const(x) => x,
    }
}

fn perform_operation(item: u32, operation: Operation) -> u64 {
    match operation {
        Operation::Add(x, y) => retrieve_value(item, x) as u64 + retrieve_value(item, y) as u64,
        Operation::Multiple(x, y) => {
            retrieve_value(item, x) as u64 * retrieve_value(item, y) as u64
        }
    }
}

fn perform_test(item: u32, test: Test) -> usize {
    if item % test.divisor == 0 {
        test.if_true
    } else {
        test.if_false
    }
}

fn process_item(item: u32, monkey: Monkey, items: &mut Vec<Vec<u32>>, reductor: u32, lcm: u32) {
    let new_item =
        ((perform_operation(item, monkey.operation) / reductor as u64) % lcm as u64) as u32;
    items[perform_test(new_item, monkey.test)].push(new_item);
}

fn take_turn(
    index: usize,
    monkey: Monkey,
    items: &mut Vec<Vec<u32>>,
    inspections: &mut Vec<u32>,
    reductor: u32,
    lcm: u32,
) {
    for item in 0..items[index].len() {
        process_item(items[index][item], monkey, items, reductor, lcm);
        inspections[index] += 1;
    }
    items[index].clear();
}

pub fn take_round(
    monkeys: &Vec<Monkey>,
    items: &mut Vec<Vec<u32>>,
    inspections: &mut Vec<u32>,
    reductor: u32,
    lcm: u32,
) {
    for (i, monkey) in monkeys.iter().enumerate() {
        take_turn(i, *monkey, items, inspections, reductor, lcm);
    }
}

pub fn calculate_common_multiple(monkey: &Vec<Monkey>) -> u32 {
    monkey.iter().map(|x| x.test.divisor).fold(1, |x, y| x * y)
}

fn parse_items(line: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let parts = line.split(':').collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        parts[1]
            .split(',')
            .map(|x| x.trim().parse::<u32>().map_err(|e| e.into()))
            .collect()
    }
}

fn parse_variable(text: &str) -> Result<Variable, Box<dyn std::error::Error>> {
    if text == "old" {
        Ok(Variable::Old)
    } else {
        Ok(Variable::Const(text.parse()?))
    }
}

fn parse_operation(line: &str) -> Result<Operation, Box<dyn std::error::Error>> {
    let parts = line.split('=').collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        let parts = parts[1].split_whitespace().collect::<Vec<_>>();
        if parts.len() != 3 {
            Err(Box::new(utils::ParseInputError))
        } else {
            if parts[1] == "*" {
                Ok(Operation::Multiple(
                    parse_variable(parts[0])?,
                    parse_variable(parts[2])?,
                ))
            } else if parts[1] == "+" {
                Ok(Operation::Add(
                    parse_variable(parts[0])?,
                    parse_variable(parts[2])?,
                ))
            } else {
                Err(Box::new(utils::ParseInputError))
            }
        }
    }
}

fn parse_test(line1: &str, line2: &str, line3: &str) -> Result<Test, Box<dyn std::error::Error>> {
    Ok(Test {
        divisor: line1
            .split_whitespace()
            .last()
            .ok_or(Box::new(utils::ParseInputError))?
            .parse()?,
        if_true: line2
            .split_whitespace()
            .last()
            .ok_or(Box::new(utils::ParseInputError))?
            .parse()?,
        if_false: line3
            .split_whitespace()
            .last()
            .ok_or(Box::new(utils::ParseInputError))?
            .parse()?,
    })
}

fn parse_monkey(lines: &str) -> Result<(Monkey, Vec<u32>), Box<dyn std::error::Error>> {
    let parts = lines.lines().collect::<Vec<_>>();
    if parts.len() != 6 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok((
            Monkey {
                operation: parse_operation(parts[2])?,
                test: parse_test(parts[3], parts[4], parts[5])?,
            },
            parse_items(parts[1])?,
        ))
    }
}

pub fn read(filename: &str) -> Result<Vec<(Monkey, Vec<u32>)>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .split("\n\n")
        .map(parse_monkey)
        .collect()
}
