use std::collections;
use std::fs;

pub fn create_directions() -> collections::VecDeque<((i32, i32), [(i32, i32); 3])> {
    let mut directions = collections::VecDeque::new();
    directions.push_back(((-1, 0), [(-1, -1), (-1, 0), (-1, 1)]));
    directions.push_back(((1, 0), [(1, -1), (1, 0), (1, 1)]));
    directions.push_back(((0, -1), [(-1, -1), (0, -1), (1, -1)]));
    directions.push_back(((0, 1), [(-1, 1), (0, 1), (1, 1)]));
    directions
}

fn consider_move(
    coords: (i32, i32),
    directions: &collections::VecDeque<((i32, i32), [(i32, i32); 3])>,
    elves: &collections::HashSet<(i32, i32)>,
) -> Option<(i32, i32)> {
    let mut should_move = false;
    for y in (coords.0 - 1)..=(coords.0 + 1) {
        for x in (coords.1 - 1)..=(coords.1 + 1) {
            if (y != coords.0 || x != coords.1) && elves.contains(&(y, x)) {
                should_move = true;
            }
        }
    }
    if should_move {
        'outer: for ((y, x), neighbors) in directions {
            for neighbor in neighbors {
                if elves.contains(&(coords.0 + neighbor.0, coords.1 + neighbor.1)) {
                    continue 'outer;
                }
            }
            return Some((coords.0 + y, coords.1 + x));
        }
        None
    } else {
        None
    }
}

fn rotate_directions(directions: &mut collections::VecDeque<((i32, i32), [(i32, i32); 3])>) {
    if let Some(elem) = directions.pop_front() {
        directions.push_back(elem);
    }
}

fn create_moves(
    directions: &collections::VecDeque<((i32, i32), [(i32, i32); 3])>,
    elves: &collections::HashSet<(i32, i32)>,
) -> (
    collections::HashMap<(i32, i32), (i32, i32)>,
    collections::HashMap<(i32, i32), u32>,
) {
    let mut result = collections::HashMap::new();
    let mut counts = collections::HashMap::new();
    for &elf in elves {
        if let Some(coords) = consider_move(elf, directions, elves) {
            result.insert(elf, coords);
            if let Some(count) = counts.get_mut(&coords) {
                *count += 1;
            } else {
                counts.insert(coords, 1);
            }
        }
    }
    (result, counts)
}

pub fn perform_round(
    directions: &mut collections::VecDeque<((i32, i32), [(i32, i32); 3])>,
    elves: collections::HashSet<(i32, i32)>,
) -> collections::HashSet<(i32, i32)> {
    let (moves, counts) = create_moves(directions, &elves);
    let mut result = collections::HashSet::new();
    for source in elves {
        if let Some(destination) = moves.get(&source) {
            if let Some(count) = counts.get(destination) {
                if count > &1 {
                    result.insert(source);
                } else {
                    result.insert(*destination);
                }
            }
        } else {
            result.insert(source);
        }
    }
    rotate_directions(directions);
    result
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|x| x == '#').collect()
}

pub fn read(filename: &str) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    let rows = fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();
    let mut result = vec![];
    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            if rows[y][x] {
                result.push((y as i32, x as i32));
            }
        }
    }
    Ok(result)
}
