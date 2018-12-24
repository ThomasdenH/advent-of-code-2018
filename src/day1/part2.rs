use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::collections::HashSet;

fn solution() -> Result<i32, Box<dyn Error>> {
    let input = File::open("./src/day1/input.txt")?;
    let buffered = BufReader::new(input);
    let changes = buffered.lines()
        .map(|line| line.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut set = HashSet::new();
    let mut total = 0;
    loop {
        for change in &changes {
            total += change;
            if set.contains(&total) {
                return Ok(total)
            } else {
                set.insert(total);
            }
        }
    }
}

#[test]
fn test_solution() {
    assert_eq!(solution().unwrap(), 80598);
}
