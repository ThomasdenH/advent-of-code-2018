use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solution() -> Result<String, Box<dyn Error>> {
    let input = File::open("./src/day2/input.txt")?;
    let buffered = BufReader::new(input);
    let lines = buffered
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                break;
            }

            let string1 = &lines[i];
            let string2 = &lines[j];

            if string1
                .chars()
                .zip(string2.chars())
                .filter(|(a, b)| a != b)
                .count()
                == 1
            {
                return Ok(string1
                    .chars()
                    .zip(string2.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _b)| a)
                    .collect::<String>());
            }
        }
    }
    panic!("Solution not found!");
}

#[test]
fn test_solution() {
    assert_eq!(solution().unwrap(), "wlkigsqyfecjqqmnxaktdrhbz");
}
