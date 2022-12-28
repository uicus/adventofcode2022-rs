pub fn solve(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().map(|x| x.iter().sum()).max().unwrap_or(0)
}
