use std::fs;

pub fn find_position(number: usize, current_order: &Vec<(usize, i64)>) -> usize {
    for (index, &elem) in current_order.iter().enumerate() {
        if elem.0 == number {
            return index;
        }
    }
    return 0;
}

pub fn find_zero(current_order: &Vec<(usize, i64)>) -> usize {
    for (index, &elem) in current_order.iter().enumerate() {
        if elem.1 == 0 {
            return index;
        }
    }
    return 0;
}

pub fn shift(position: usize, current_order: &mut Vec<(usize, i64)>) {
    let elem = current_order.remove(position);
    let new_position = (position as i64 + elem.1).rem_euclid(current_order.len() as i64);
    current_order.insert(new_position as usize, elem);
}

pub fn read(filename: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(|x| x.parse::<i32>().map_err(|e| e.into()))
        .collect()
}
