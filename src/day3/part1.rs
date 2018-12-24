use super::Claim;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn overlapping(claims: &Vec<Claim>) -> usize {
    let mut tiles: HashMap<(u32, u32), u32> = HashMap::new();
    for claim in claims {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                let coord = (x, y);
                tiles.insert(
                    coord,
                    1 + match tiles.get(&coord) {
                        Some(num) => *num,
                        None => 0,
                    },
                );
            }
        }
    }
    tiles.values().filter(|val| **val >= 2).count()
}

#[test]
fn test_overlapping() {
    let claims = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
        .iter()
        .map(|l| l.parse::<Claim>().unwrap())
        .collect();
    assert_eq!(overlapping(&claims), 4);
}

fn solution() -> Result<usize, Box<dyn Error>> {
    let file = File::open("./src/day3/input.txt")?;
    let reader = BufReader::new(file);
    let claims = reader
        .lines()
        .map(|s| s.unwrap().parse::<Claim>().unwrap())
        .collect::<Vec<Claim>>();
    Ok(overlapping(&claims))
}

#[test]
fn test_solution() {
    assert_eq!(solution().unwrap(), 107043);
}
