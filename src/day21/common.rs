use crate::utils;
use std::collections;
use std::fs;

#[derive(Clone)]
pub enum Operation {
    Const(i64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

pub struct Monkey {
    pub name: String,
    pub operation: Operation,
}

pub fn value(name: &String, all_monkeys: &collections::HashMap<String, Operation>) -> i64 {
    if let Some(operation) = all_monkeys.get(name) {
        let result = match operation {
            Operation::Const(x) => *x,
            Operation::Add(x, y) => value(x, all_monkeys) + value(y, all_monkeys),
            Operation::Subtract(x, y) => value(x, all_monkeys) - value(y, all_monkeys),
            Operation::Multiply(x, y) => value(x, all_monkeys) * value(y, all_monkeys),
            Operation::Divide(x, y) => value(x, all_monkeys) / value(y, all_monkeys),
        };
        return result;
    }
    return 0;
}

fn parse_operation(text: &str) -> Result<Operation, Box<dyn std::error::Error>> {
    let parts = text.split_whitespace().collect::<Vec<_>>();
    if parts.len() == 1 {
        Ok(Operation::Const(parts[0].parse()?))
    } else if parts.len() == 3 {
        match parts[1] {
            "+" => Ok(Operation::Add(parts[0].to_string(), parts[2].to_string())),
            "-" => Ok(Operation::Subtract(
                parts[0].to_string(),
                parts[2].to_string(),
            )),
            "*" => Ok(Operation::Multiply(
                parts[0].to_string(),
                parts[2].to_string(),
            )),
            "/" => Ok(Operation::Divide(
                parts[0].to_string(),
                parts[2].to_string(),
            )),
            _ => Err(Box::new(utils::ParseInputError)),
        }
    } else {
        Err(Box::new(utils::ParseInputError))
    }
}

fn parse_line(line: &str) -> Result<Monkey, Box<dyn std::error::Error>> {
    let parts = line.split(":").map(|x| x.trim()).collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok(Monkey {
            name: parts[0].to_string(),
            operation: parse_operation(parts[1])?,
        })
    }
}

pub fn read(filename: &str) -> Result<Vec<Monkey>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
