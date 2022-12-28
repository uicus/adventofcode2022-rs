use super::common;

fn decode(number: &Vec<common::Digit>) -> i64 {
    let mut result = 0;
    for digit in number {
        result *= 5;
        result += match digit {
            common::Digit::Two => 2,
            common::Digit::One => 1,
            common::Digit::Zero => 0,
            common::Digit::Minus => -1,
            common::Digit::DoubleMinus => -2,
        };
    }
    result
}

fn encode(mut number: i64) -> Vec<common::Digit> {
    let mut result = vec![];
    while number != 0 {
        let digit = match number % 5 {
            0 => common::Digit::Zero,
            1 => common::Digit::One,
            2 => common::Digit::Two,
            3 => common::Digit::DoubleMinus,
            4 => common::Digit::Minus,
            _ => unreachable!(),
        };
        let carryover = if number % 5 == 3 || number % 5 == 4 {
            1
        } else {
            0
        };
        number = number / 5 + carryover;
        result.push(digit);
    }
    result.reverse();
    result
}

fn digits_to_string(number: &Vec<common::Digit>) -> String {
    number
        .iter()
        .map(|digit| match digit {
            common::Digit::Two => '2',
            common::Digit::One => '1',
            common::Digit::Zero => '0',
            common::Digit::Minus => '-',
            common::Digit::DoubleMinus => '=',
        })
        .collect()
}

pub fn solve(input: &Vec<Vec<common::Digit>>) -> String {
    digits_to_string(&encode(input.iter().map(decode).sum()))
}
