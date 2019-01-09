use aoc_runner_derive::*;
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_frequencies(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[isize]) -> isize {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[isize]) -> isize {
    let mut set = HashSet::new();
    let mut total = 0;
    loop {
        for change in input {
            total += change;
            if set.contains(&total) {
                return total;
            } else {
                set.insert(total);
            }
        }
    }
}

#[test]
fn test_part1_example() {
    let input = "+1\n-2\n+3\n+1";
    let freqs = input_frequencies(&input);
    let result = part1(&freqs);
    assert_eq!(result, 3);
}

#[test]
fn test_part1() {
    let input = crate::util::read_file_to_string("./input/2018/day1.txt");
    let freqs = input_frequencies(&input);
    let result = part1(&freqs);
    assert_eq!(result, 595);
}

#[test]
fn test_part2_example() {
    let input = "+1\n-2\n+3\n+1";
    let freqs = input_frequencies(&input);
    let result = part2(&freqs);
    assert_eq!(result, 2);
}

#[test]
fn test_part2() {
    let input = crate::util::read_file_to_string("./input/2018/day1.txt");
    let freqs = input_frequencies(&input);
    let result = part2(&freqs);
    assert_eq!(result, 80598);
}
