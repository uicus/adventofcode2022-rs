use super::common;

fn operate(position: &mut Vec<Vec<char>>, operation: &common::Operation) {
    let mut temp = vec![];
    for _ in 0..operation.count {
        if let Some(elem) = position[operation.from - 1].pop() {
            temp.push(elem);
        }
    }
    for elem in temp.iter().rev() {
        position[operation.to - 1].push(*elem);
    }
}

pub fn solve(input: &(Vec<Vec<char>>, Vec<common::Operation>)) -> String {
    let mut position = input.0.clone();
    for operation in &input.1 {
        operate(&mut position, &operation);
    }
    position.iter().filter_map(|x| x.last()).collect()
}
