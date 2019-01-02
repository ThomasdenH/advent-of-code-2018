#![forbid(unsafe_code)]

#[cfg(test)]
mod day1;

#[cfg(test)]
mod day2;

#[cfg(test)]
mod day3;

mod day4;

use crate::day4::part1::solution;


fn main() {
    println!("{}", solution().unwrap());
}
