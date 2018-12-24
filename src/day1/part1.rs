use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

fn solution() -> Result<i32, Box<dyn Error>> {
    let input = File::open("./src/day1/input.txt")?;
    let buffered = BufReader::new(input);
    Ok(buffered.lines()
        .map(|line| line.unwrap().trim().parse::<i32>().unwrap())
        .sum())
}

#[test]
fn test_solution() {
    assert_eq!(solution().unwrap(), 595);
}
