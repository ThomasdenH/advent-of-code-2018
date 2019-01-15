use aoc_runner_derive::*;
use core::num::ParseIntError;
use core::str::FromStr;
use std::collections::HashMap;

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
            .split(',')
            .collect::<Vec<_>>();
        let left = coord[0].parse::<u32>()?;
        let top = coord[1].parse::<u32>()?;
        let size = parts[3].split('x').collect::<Vec<_>>();
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

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(|s| s.parse::<Claim>().unwrap())
        .collect::<Vec<Claim>>()
}

#[aoc(day3, part1)]
fn part1(claims: &[Claim]) -> usize {
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
fn test_part1_example() {
    let claims = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
        .iter()
        .map(|l| l.parse::<Claim>().unwrap())
        .collect::<Vec<Claim>>();
    assert_eq!(part1(&claims), 4);
}

#[aoc(day3, part2)]
fn part2(claims: &[Claim]) -> u32 {
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
            true
        })
        .unwrap()
        .id
}

#[test]
fn test_part2_example() {
    let claims = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
        .iter()
        .map(|l| l.parse::<Claim>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(part2(&claims), 3);
}

#[test]
fn test_part1() {
    let input_string = crate::util::read_file_to_string("./input/2018/day3.txt");
    let input = generator(&input_string);
    let result = part1(&input);
    assert_eq!(result, 107_043);
}

#[test]
fn test_part2() {
    let input_string = crate::util::read_file_to_string("./input/2018/day3.txt");
    let input = generator(&input_string);
    let result = part2(&input);
    assert_eq!(result, 346);
}
