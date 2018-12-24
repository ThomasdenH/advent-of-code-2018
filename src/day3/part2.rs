use super::Claim;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn non_overlapping(claims: &Vec<Claim>) -> u32 {
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

    claims
        .iter()
        .find(|claim| {
            for x in claim.left..claim.left + claim.width {
                for y in claim.top..claim.top + claim.height {
                    if tiles.get(&(x, y)) != Some(&1) {
                        return false;
                    }
                }
            }
            return true;
        })
        .unwrap()
        .id
}

#[test]
fn test_overlapping() {
    let claims = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
        .iter()
        .map(|l| l.parse::<Claim>().unwrap())
        .collect();
    assert_eq!(non_overlapping(&claims), 3);
}

fn solution() -> Result<u32, Box<dyn Error>> {
    let file = File::open("./src/day3/input.txt")?;
    let reader = BufReader::new(file);
    let claims = reader
        .lines()
        .map(|s| s.unwrap().parse::<Claim>().unwrap())
        .collect::<Vec<Claim>>();
    Ok(non_overlapping(&claims))
}

#[test]
fn test_solution() {
    assert_eq!(solution().unwrap(), 346);
}
