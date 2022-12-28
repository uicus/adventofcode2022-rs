use crate::utils;
use std::fs;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum OpponentMove {
    A,
    B,
    C,
}

impl FromStr for OpponentMove {
    type Err = utils::ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            Err(utils::ParseInputError)
        } else {
            match s.as_bytes()[0] as char {
                'A' => Ok(OpponentMove::A),
                'B' => Ok(OpponentMove::B),
                'C' => Ok(OpponentMove::C),
                _ => Err(utils::ParseInputError),
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum YourMove {
    X,
    Y,
    Z,
}

impl FromStr for YourMove {
    type Err = utils::ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            Err(utils::ParseInputError)
        } else {
            match s.as_bytes()[0] as char {
                'X' => Ok(YourMove::X),
                'Y' => Ok(YourMove::Y),
                'Z' => Ok(YourMove::Z),
                _ => Err(utils::ParseInputError),
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

fn figure_score(figure: Move) -> u32 {
    match figure {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

pub fn opponent_correspondence(opponent_move: OpponentMove) -> Move {
    match opponent_move {
        OpponentMove::A => Move::Rock,
        OpponentMove::B => Move::Paper,
        OpponentMove::C => Move::Scissors,
    }
}

pub fn your_correspondence(your_move: YourMove) -> Move {
    match your_move {
        YourMove::X => Move::Rock,
        YourMove::Y => Move::Paper,
        YourMove::Z => Move::Scissors,
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum RoundResult {
    Win,
    Lose,
    Tie,
}

pub fn result_correspondence(your_move: YourMove) -> RoundResult {
    match your_move {
        YourMove::X => RoundResult::Lose,
        YourMove::Y => RoundResult::Tie,
        YourMove::Z => RoundResult::Win,
    }
}

fn result_score(result: RoundResult) -> u32 {
    match result {
        RoundResult::Win => 6,
        RoundResult::Lose => 0,
        RoundResult::Tie => 3,
    }
}

pub fn round_result(move1: Move, move2: Move) -> RoundResult {
    match (move1, move2) {
        (Move::Rock, Move::Rock) => RoundResult::Tie,
        (Move::Rock, Move::Paper) => RoundResult::Win,
        (Move::Rock, Move::Scissors) => RoundResult::Lose,
        (Move::Paper, Move::Rock) => RoundResult::Lose,
        (Move::Paper, Move::Paper) => RoundResult::Tie,
        (Move::Paper, Move::Scissors) => RoundResult::Win,
        (Move::Scissors, Move::Rock) => RoundResult::Win,
        (Move::Scissors, Move::Paper) => RoundResult::Lose,
        (Move::Scissors, Move::Scissors) => RoundResult::Tie,
    }
}

pub fn count_result(input: &Vec<(OpponentMove, YourMove)>) -> u32 {
    input
        .iter()
        .map(|&(opponent, your)| {
            result_score(round_result(
                opponent_correspondence(opponent),
                your_correspondence(your),
            )) + figure_score(your_correspondence(your))
        })
        .sum()
}

fn parse_line(line: &str) -> Result<(OpponentMove, YourMove), Box<dyn std::error::Error>> {
    let letters = line.split(' ').take(2).collect::<Vec<&str>>();
    if letters.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok((letters[0].parse()?, letters[1].parse()?))
    }
}

pub fn read(filename: &str) -> Result<Vec<(OpponentMove, YourMove)>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
