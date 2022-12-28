use super::common;

fn move_once(position: &mut Vec<Vec<char>>, from: usize, to: usize) {
    if let Some(elem) = position[from - 1].pop() {
        position[to - 1].push(elem);
    }
}

fn operate(position: &mut Vec<Vec<char>>, operation: &common::Operation) {
    for _ in 0..operation.count {
        move_once(position, operation.from, operation.to);
    }
}

pub fn solve(input: &(Vec<Vec<char>>, Vec<common::Operation>)) -> String {
    let mut position = input.0.clone();
    for operation in &input.1 {
        operate(&mut position, &operation);
    }
    position.iter().filter_map(|x| x.last()).collect()
}
