use super::common;

pub fn solve(input: &common::Grid) -> u32 {
    common::traverse(common::find_end(input), input, |cell| {
        if let common::Cell::Start = cell {
            true
        } else {
            false
        }
    })
}
