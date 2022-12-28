fn inside(segment1: (u32, u32), segment2: (u32, u32)) -> bool {
    segment1.0 >= segment2.0 && segment1.1 <= segment2.1
}

fn symmetric_inside(segment1: (u32, u32), segment2: (u32, u32)) -> bool {
    inside(segment1, segment2) || inside(segment2, segment1)
}

pub fn solve(input: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    input
        .iter()
        .filter(|(x, y)| symmetric_inside(*x, *y))
        .count() as u32
}
