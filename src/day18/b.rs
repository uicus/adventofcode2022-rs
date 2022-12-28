use super::common;
use std::collections;

fn limits(points: &Vec<(i32, i32, i32)>) -> (i32, i32, i32) {
    let mut max_1 = 0;
    let mut max_2 = 0;
    let mut max_3 = 0;
    for &(x, y, z) in points {
        if x > max_1 {
            max_1 = x;
        }
        if y > max_2 {
            max_2 = y;
        }
        if z > max_3 {
            max_3 = z;
        }
    }
    (max_1 + 2, max_2 + 2, max_3 + 2)
}

fn fill(barriers: &Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let barriers_set = barriers.iter().collect::<collections::HashSet<_>>();
    let mut outside = collections::HashSet::new();
    let mut stack = vec![(0, 0, 0)];
    let edges = limits(barriers);
    while let Some((x, y, z)) = stack.pop() {
        outside.insert((x, y, z));
        for (next_x, next_y, next_z) in [
            (x, y, z - 1),
            (x, y, z + 1),
            (x, y - 1, z),
            (x, y + 1, z),
            (x - 1, y, z),
            (x + 1, y, z),
        ] {
            if next_x >= 0
                && next_x < edges.0
                && next_y >= 0
                && next_y < edges.1
                && next_z >= 0
                && next_z < edges.2
                && !barriers_set.contains(&(next_x, next_y, next_z))
                && !outside.contains(&(next_x, next_y, next_z))
            {
                stack.push((next_x, next_y, next_z));
            }
        }
    }
    let mut inside = vec![];
    for x in 0..edges.0 {
        for y in 0..edges.1 {
            for z in 0..edges.2 {
                if !barriers_set.contains(&(x, y, z)) && !outside.contains(&(x, y, z)) {
                    inside.push((x, y, z))
                }
            }
        }
    }
    inside
}

pub fn solve(input: &Vec<(i32, i32, i32)>) -> usize {
    let mut cubes = input.clone();
    cubes.extend(fill(input));
    common::count_outside(&cubes)
}
