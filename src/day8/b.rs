fn go(
    heights: &Vec<Vec<u32>>,
    point: (isize, isize),
    shift: (isize, isize),
    limits: (isize, isize),
) -> u32 {
    let start_height = heights[point.0 as usize][point.1 as usize];
    let mut result = 0;
    let mut current_x = point.0 + shift.0;
    let mut current_y = point.1 + shift.1;
    while current_x >= 0 && current_y >= 0 && current_x < limits.0 && current_y < limits.1 {
        result += 1;
        if heights[current_x as usize][current_y as usize] >= start_height {
            break;
        }
        current_x += shift.0;
        current_y += shift.1;
    }
    result
}

fn score(heights: &Vec<Vec<u32>>, point: (isize, isize), limits: (isize, isize)) -> u32 {
    go(heights, point, (1, 0), limits)
        * go(heights, point, (-1, 0), limits)
        * go(heights, point, (0, 1), limits)
        * go(heights, point, (0, -1), limits)
}

pub fn solve(input: &Vec<Vec<u32>>) -> u32 {
    let limits = (input.len() as isize, input[0].len() as isize);
    let mut current_max = 0;
    for x in 0..limits.0 {
        for y in 0..limits.1 {
            let candidate = score(input, (x, y), limits);
            if candidate > current_max {
                current_max = candidate;
            }
        }
    }
    current_max
}
