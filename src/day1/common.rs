use std::fs;

pub fn read(filename: &str) -> Result<Vec<Vec<u32>>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|y| y.parse::<u32>().map_err(|e| e.into()))
                .collect()
        })
        .collect()
}
