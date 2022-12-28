fn overlap(segment1: (u32, u32), segment2: (u32, u32)) -> bool {
    segment1.0 <= segment2.1 && segment1.1 >= segment2.0
}

pub fn solve(input: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    input.iter().filter(|(x, y)| overlap(*x, *y)).count() as u32
}
