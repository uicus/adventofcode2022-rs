use super::common;
use std::collections;

fn measure_edge(grid: &common::Grid) -> usize {
    std::cmp::max(grid.cells.len(), grid.cells[0].len()) / 4
}

fn create_mapping(
    edge1: &Vec<(i32, i32)>,
    edge2: &Vec<(i32, i32)>,
    direction1: (i32, i32),
    direction2: (i32, i32),
) -> Vec<(((i32, i32), (i32, i32)), ((i32, i32), (i32, i32)))> {
    std::iter::zip(edge1, edge2)
        .map(|(point1, point2)| {
            (
                (
                    (point1.0 + direction1.0, point1.1 + direction1.1),
                    direction1,
                ),
                (*point2, (-direction2.0, -direction2.1)),
            )
        })
        .collect()
}

fn create_symmetric_mapping(
    edge1: &Vec<(i32, i32)>,
    edge2: &Vec<(i32, i32)>,
    direction1: (i32, i32),
    direction2: (i32, i32),
) -> Vec<(((i32, i32), (i32, i32)), ((i32, i32), (i32, i32)))> {
    let mut result = create_mapping(edge1, edge2, direction1, direction2);
    result.append(&mut create_mapping(edge2, edge1, direction2, direction1));
    result
}

fn create_edge(
    start: (i32, i32),
    end: (i32, i32),
    direction: (i32, i32),
    size: usize,
) -> Vec<(i32, i32)> {
    if start.0 == end.0 {
        let y = start.0 * size as i32 - if direction.0 > 0 { 1 } else { 0 };
        if start.1 < end.1 {
            ((start.1 * size as i32)..(end.1 * size as i32))
                .map(|x| (y, x))
                .collect()
        } else {
            ((end.1 * size as i32)..(start.1 * size as i32))
                .rev()
                .map(|x| (y, x))
                .collect()
        }
    } else {
        let y = start.1 * size as i32 - if direction.1 > 0 { 1 } else { 0 };
        if start.0 < end.0 {
            ((start.0 * size as i32)..(end.0 * size as i32))
                .map(|x| (x, y))
                .collect()
        } else {
            ((end.0 * size as i32)..(start.0 * size as i32))
                .rev()
                .map(|x| (x, y))
                .collect()
        }
    }
}

fn create_mapping_by_points(
    start1: (i32, i32),
    end1: (i32, i32),
    direction1: (i32, i32),
    start2: (i32, i32),
    end2: (i32, i32),
    direction2: (i32, i32),
    size: usize,
) -> Vec<(((i32, i32), (i32, i32)), ((i32, i32), (i32, i32)))> {
    create_symmetric_mapping(
        &create_edge(start1, end1, direction1, size),
        &create_edge(start2, end2, direction2, size),
        direction1,
        direction2,
    )
}

fn add_to_map(
    map: &mut collections::HashMap<((i32, i32), (i32, i32)), ((i32, i32), (i32, i32))>,
    list: &Vec<(((i32, i32), (i32, i32)), ((i32, i32), (i32, i32)))>,
) {
    for &(key, value) in list {
        map.insert(key, value);
    }
}

fn create_edges(
    grid: &common::Grid,
) -> collections::HashMap<((i32, i32), (i32, i32)), ((i32, i32), (i32, i32))> {
    let edge_size = measure_edge(grid);
    let mut result = collections::HashMap::new();
    if edge_size < 10 {
        add_to_map(
            &mut result,
            &create_mapping_by_points((1, 1), (1, 2), (-1, 0), (0, 2), (1, 2), (0, -1), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((1, 0), (1, 1), (-1, 0), (0, 3), (0, 2), (-1, 0), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((2, 0), (2, 1), (1, 0), (3, 3), (3, 2), (1, 0), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((2, 1), (2, 2), (1, 0), (3, 2), (2, 2), (0, -1), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((1, 0), (2, 0), (0, -1), (3, 4), (3, 3), (1, 0), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((1, 3), (2, 3), (0, 1), (2, 4), (2, 3), (-1, 0), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((0, 3), (1, 3), (0, 1), (3, 4), (2, 4), (0, 1), edge_size),
        );
    } else {
        add_to_map(
            &mut result,
            &create_mapping_by_points((1, 1), (2, 1), (0, -1), (2, 0), (2, 1), (-1, 0), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((1, 2), (1, 3), (1, 0), (1, 2), (2, 2), (0, 1), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((3, 1), (3, 2), (1, 0), (3, 1), (4, 1), (0, 1), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((2, 0), (3, 0), (0, -1), (1, 1), (0, 1), (0, -1), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((2, 2), (3, 2), (0, 1), (1, 3), (0, 3), (0, 1), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((3, 0), (4, 0), (0, -1), (0, 1), (0, 2), (-1, 0), edge_size),
        );
        add_to_map(
            &mut result,
            &create_mapping_by_points((4, 0), (4, 1), (1, 0), (0, 2), (0, 3), (-1, 0), edge_size),
        );
    }
    result
}

fn move_once_forward(
    coords: (i32, i32),
    direction: (i32, i32),
    grid: &common::Grid,
    edges: &collections::HashMap<((i32, i32), (i32, i32)), ((i32, i32), (i32, i32))>,
) -> ((i32, i32), (i32, i32)) {
    let new_coords = (coords.0 + direction.0, coords.1 + direction.1);
    if common::outside(new_coords, grid) {
        if let Some(result) = edges.get(&(new_coords, direction)) {
            return *result;
        } else {
            println!(
                "({}, {}), ({}, {})",
                coords.0, coords.1, direction.0, direction.1
            );
            println!("{}, {}", grid.cells.len(), grid.cells[0].len());
            println!(
                "({}, {}), ({}, {})",
                new_coords.0, new_coords.1, direction.0, direction.1
            );
            panic!("hhh");
        }
    } else {
        (new_coords, direction)
    }
}

fn move_forward(
    mut coords: (i32, i32),
    count: i32,
    mut direction: (i32, i32),
    grid: &common::Grid,
    edges: &collections::HashMap<((i32, i32), (i32, i32)), ((i32, i32), (i32, i32))>,
) -> ((i32, i32), (i32, i32)) {
    for _ in 0..count {
        let (new_coords, new_direction) = move_once_forward(coords, direction, grid, edges);
        if grid.cells[new_coords.0 as usize][new_coords.1 as usize] == common::Cell::Wall {
            return (coords, direction);
        }
        coords = new_coords;
        direction = new_direction;
    }
    (coords, direction)
}

pub fn solve(input: &(common::Grid, Vec<common::Instruction>)) -> i32 {
    let edges = create_edges(&input.0);
    let mut current_coords = common::start_coords(&input.0);
    let mut current_direction = (0, 1);
    for instruction in &input.1 {
        match instruction {
            &common::Instruction::Forward(x) => {
                (current_coords, current_direction) =
                    move_forward(current_coords, x, current_direction, &input.0, &edges)
            }
            common::Instruction::Left => current_direction = common::rotate_left(current_direction),
            common::Instruction::Right => {
                current_direction = common::rotate_right(current_direction)
            }
        }
    }
    1000 * (current_coords.0 + 1)
        + 4 * (current_coords.1 + 1)
        + common::direction_value(current_direction)
}
