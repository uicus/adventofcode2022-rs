use crate::utils;
use std::cmp;
use std::collections;
use std::fs;

#[derive(Clone)]
pub enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

pub fn compare(packet1: &Packet, packet2: &Packet) -> cmp::Ordering {
    match (packet1, packet2) {
        (Packet::Integer(x), Packet::Integer(y)) => x.cmp(y),
        (Packet::List(x), Packet::List(y)) => {
            for (el1, el2) in x.iter().zip(y.iter()) {
                let potential_result = compare(el1, el2);
                if potential_result != cmp::Ordering::Equal {
                    return potential_result;
                }
            }
            x.len().cmp(&y.len())
        }
        (Packet::Integer(_), Packet::List(_)) => {
            compare(&Packet::List(vec![packet1.clone()]), packet2)
        }
        (Packet::List(_), Packet::Integer(_)) => {
            compare(packet1, &Packet::List(vec![packet2.clone()]))
        }
    }
}

fn parse_integer(
    text: &mut collections::VecDeque<char>,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut result = 0;
    if !text.front().unwrap_or(&'?').is_digit(10) {
        return Err(Box::new(utils::ParseInputError));
    }
    while text.front().unwrap_or(&'?').is_digit(10) {
        result *= 10;
        result += text.pop_front().unwrap_or('?') as u32 - '0' as u32;
    }
    Ok(result)
}

fn parse_list(
    text: &mut collections::VecDeque<char>,
) -> Result<Vec<Packet>, Box<dyn std::error::Error>> {
    let mut result = vec![];
    if text.front() != Some(&'[') {
        return Err(Box::new(utils::ParseInputError));
    }
    text.pop_front();
    while text.front() != Some(&']') {
        result.push(parse_packet(text)?);
        if text.front() == Some(&',') {
            text.pop_front();
        }
    }
    text.pop_front();
    Ok(result)
}

fn parse_packet(
    text: &mut collections::VecDeque<char>,
) -> Result<Packet, Box<dyn std::error::Error>> {
    if text.len() < 1 {
        Err(Box::new(utils::ParseInputError))
    } else if text.front() == Some(&'[') {
        Ok(Packet::List(parse_list(text)?))
    } else if text.front().unwrap_or(&'?').is_digit(10) {
        Ok(Packet::Integer(parse_integer(text)?))
    } else {
        Err(Box::new(utils::ParseInputError))
    }
}

fn parse_part(lines: &str) -> Result<(Packet, Packet), Box<dyn std::error::Error>> {
    let lines = lines.lines().collect::<Vec<_>>();
    if lines.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok((
            parse_packet(&mut lines[0].chars().collect())?,
            parse_packet(&mut lines[1].chars().collect())?,
        ))
    }
}

pub fn read(filename: &str) -> Result<Vec<(Packet, Packet)>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .split("\n\n")
        .map(parse_part)
        .collect()
}
