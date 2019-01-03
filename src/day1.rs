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
