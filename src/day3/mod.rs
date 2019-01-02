use core::num::ParseIntError;
use std::str::FromStr;

pub mod part1;
mod part2;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let id = parts[0].split_at(1).1.parse::<u32>()?;
        let coord = parts[2]
            .split_at(parts[2].len() - 1)
            .0
            .split(",")
            .collect::<Vec<_>>();
        let left = coord[0].parse::<u32>()?;
        let top = coord[1].parse::<u32>()?;
        let size = parts[3].split("x").collect::<Vec<_>>();
        let width = size[0].parse::<u32>()?;
        let height = size[1].parse::<u32>()?;
        Ok(Claim {
            id,
            left,
            top,
            width,
            height,
        })
    }
}

#[test]
fn test_claim_parse() {
    let input = "#123 @ 3,2: 5x4\n";
    assert_eq!(
        input.parse::<Claim>(),
        Ok(Claim {
            id: 123,
            left: 3,
            top: 2,
            width: 5,
            height: 4
        })
    );
}
