fn cast_visibility(
    visible_so_far: &mut Vec<Vec<bool>>,
    heights: &Vec<Vec<u32>>,
    shift: (isize, isize),
    starting_points: Vec<(isize, isize)>,
    limits: (isize, isize),
) {
    for (x, y) in starting_points {
        let mut current_x = x;
        let mut current_y = y;
        let mut current_max: i32 = -1;
        while current_x >= 0 && current_y >= 0 && current_x < limits.0 && current_y < limits.1 {
            let current_height = heights[current_x as usize][current_y as usize] as i32;
            if current_max < current_height {
                visible_so_far[current_x as usize][current_y as usize] = true;
                current_max = current_height;
            }
            current_x += shift.0;
            current_y += shift.1;
        }
    }
}

fn cast_north_visibility(
    visible_so_far: &mut Vec<Vec<bool>>,
    heights: &Vec<Vec<u32>>,
    limits: (isize, isize),
) {
    cast_visibility(
        visible_so_far,
        heights,
        (1, 0),
        (0..limits.1).map(|y| (0, y)).collect(),
        limits,
    )
}

fn cast_south_visibility(
    visible_so_far: &mut Vec<Vec<bool>>,
    heights: &Vec<Vec<u32>>,
    limits: (isize, isize),
) {
    cast_visibility(
        visible_so_far,
        heights,
        (-1, 0),
        (0..limits.1).map(|y| (limits.0 - 1, y)).collect(),
        limits,
    )
}

fn cast_west_visibility(
    visible_so_far: &mut Vec<Vec<bool>>,
    heights: &Vec<Vec<u32>>,
    limits: (isize, isize),
) {
    cast_visibility(
        visible_so_far,
        heights,
        (0, 1),
        (0..limits.0).map(|x| (x, 0)).collect(),
        limits,
    )
}

fn cast_east_visibility(
    visible_so_far: &mut Vec<Vec<bool>>,
    heights: &Vec<Vec<u32>>,
    limits: (isize, isize),
) {
    cast_visibility(
        visible_so_far,
        heights,
        (0, -1),
        (0..limits.0).map(|x| (x, limits.1 - 1)).collect(),
        limits,
    )
}

pub fn solve(input: &Vec<Vec<u32>>) -> u32 {
    let limits = (input.len() as isize, input[0].len() as isize);
    let mut visibility = vec![vec![false; limits.1 as usize]; limits.0 as usize];
    cast_north_visibility(&mut visibility, input, limits);
    cast_south_visibility(&mut visibility, input, limits);
    cast_east_visibility(&mut visibility, input, limits);
    cast_west_visibility(&mut visibility, input, limits);
    visibility
        .iter()
        .map(|x| x.iter().map(|&x| x as u32).sum::<u32>())
        .sum()
}
