use crate::utils;
use std::fs;

pub struct Graph {
    vertices: Vec<(u32, Vec<usize>)>,
}

pub fn sum_weights(graph: &Graph) -> u32 {
    graph.vertices.iter().map(|x| x.0).sum()
}

fn set_opened(index: usize, opened_so_far: u64) -> u64 {
    opened_so_far | (1 << index)
}

fn get_opened(index: usize, opened_so_far: u64) -> bool {
    opened_so_far & (1 << index) > 0
}

pub fn perform_step(
    opened: u64,
    place: usize,
    previous_place: usize,
    result_per_minute: u32,
    graph: &Graph,
) -> Vec<(u64, usize, u32)> {
    let mut possible_steps = vec![];
    if graph.vertices[place].0 > 0 && !get_opened(place, opened) {
        possible_steps.push((
            set_opened(place, opened),
            place,
            result_per_minute + graph.vertices[place].0,
        ));
    }
    for target in &graph.vertices[place].1 {
        if *target != previous_place {
            possible_steps.push((opened, *target, result_per_minute));
        }
    }
    possible_steps
}

fn parse_line(line: &str) -> Result<(String, u32, Vec<String>), Box<dyn std::error::Error>> {
    let parts = line.split(";").collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        let first_parts = parts[0].split_whitespace().collect::<Vec<_>>();
        if first_parts.len() != 2 {
            Err(Box::new(utils::ParseInputError))
        } else {
            Ok((
                first_parts[0].to_string(),
                first_parts[1].parse()?,
                parts[1]
                    .split(",")
                    .map(|x| x.trim())
                    .map(|x| x.to_string())
                    .collect(),
            ))
        }
    }
}

fn get_index(destination: &str, all_vertices: &Vec<(String, u32, Vec<String>)>) -> usize {
    for (index, (label, _, _)) in all_vertices.iter().enumerate() {
        if label == destination {
            return index;
        }
    }
    0
}

pub fn read(filename: &str) -> Result<(Graph, usize), Box<dyn std::error::Error>> {
    let raw: Vec<(String, u32, Vec<String>)> = fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect::<Result<_, _>>()?;
    Ok((
        Graph {
            vertices: raw
                .iter()
                .map(|(_, value, targets)| {
                    (
                        *value,
                        targets.iter().map(|label| get_index(label, &raw)).collect(),
                    )
                })
                .collect(),
        },
        get_index("AA", &raw),
    ))
}
