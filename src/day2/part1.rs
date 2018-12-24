use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

fn checksum(id: String) -> (bool, bool) {
    let mut frequencies = [0; 26];
    for char in id.trim().chars() {
        let index = (char.to_digit(36).unwrap() - 10) as usize;
        frequencies[index] += 1;
    }
    let contains2 = frequencies.contains(&2);
    let contains3 = frequencies.contains(&3);
    (contains2, contains3)
}

fn to_binary_digit(b: bool) -> u32 {
    match b {
        true => 1,
        false => 0
    }
}

fn solution() -> Result<u32, Box<dyn Error>> {
    let input = File::open("./src/day2/input.txt")?;
    let buffered = BufReader::new(input);
    let lines = buffered.lines().map(|line| line.unwrap());

    let (a, b) = lines
        .filter(|line| line.len() > 0)
        .map(checksum)
        .map(|(a, b)| (to_binary_digit(a), to_binary_digit(b)))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    Ok(a * b)
}

#[test]
fn test_solution() {
    assert_eq!(solution().unwrap(), 7776);
}
