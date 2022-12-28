pub fn solve(input: &Vec<Vec<u32>>) -> u32 {
    let mut sums = input.iter().map(|x| x.iter().sum()).collect::<Vec<u32>>();
    sums.sort_unstable();
    sums.iter().rev().take(3).sum()
}
