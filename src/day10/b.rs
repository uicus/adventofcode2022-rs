use super::common;

fn pixel(cycle: u32, register: i32) -> char {
    let position = ((cycle - 1) % 40) as i32;
    if position >= register - 1 && position <= register + 1 {
        '#'
    } else {
        ' '
    }
}

pub fn solve(input: &Vec<common::Command>) -> String {
    let mut result = String::new();
    let mut register = 1;
    let mut cycle = 1;
    for line in input {
        for _ in 0..common::cycles(*line) {
            if cycle % 40 == 1 {
                result.push('\n')
            }
            result.push(pixel(cycle, register));
            cycle += 1;
        }
        register = common::change_register(*line, register);
    }
    result
}
