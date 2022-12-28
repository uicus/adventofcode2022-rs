use crate::utils;
use std::fs;

#[derive(Clone)]
pub struct Operation {
    pub from: usize,
    pub to: usize,
    pub count: usize,
}

fn parse_position_line(text: &str) -> Vec<Option<char>> {
    text.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|x| if x[1] == ' ' { None } else { Some(x[1]) })
        .collect()
}

fn parse_position(text: &str) -> Vec<Vec<char>> {
    let rows = text.lines().map(parse_position_line).collect::<Vec<_>>();
    let width = rows[0].len();
    let mut result = vec![vec![]; width];
    for row in rows.iter().rev().skip(1) {
        for (index, elem) in row.iter().enumerate() {
            if let &Some(e) = elem {
                result[index].push(e);
            }
        }
    }
    result
}

fn parse_operation(text: &str) -> Result<Operation, Box<dyn std::error::Error>> {
    let words = text.split_whitespace().collect::<Vec<_>>();
    Ok(Operation {
        from: words[3].parse()?,
        to: words[5].parse()?,
        count: words[1].parse()?,
    })
}

pub fn read(
    filename: &str,
) -> Result<(Vec<Vec<char>>, Vec<Operation>), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(filename)?;
    let parts = text.split("\n\n").collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(Box::new(utils::ParseInputError));
    }
    Ok((
        parse_position(parts[0]),
        parts[1]
            .lines()
            .map(parse_operation)
            .collect::<Result<_, _>>()?,
    ))
}
