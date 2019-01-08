use aoc_runner_derive::*;

struct Polymer(String);

fn char_reduces_with(c1: char, c2: Option<&char>) -> bool {
    match c2 {
        Some(&c2) => c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2,
        None => false,
    }
}

impl Polymer {
    fn new(s: impl Into<String>) -> Polymer {
        Polymer(s.into())
    }

    fn reduced_len(&self) -> usize {
        let mut reduced = Vec::new();
        for c in self.0.chars() {
            if char_reduces_with(c, reduced.last()) {
                reduced.pop();
            } else {
                reduced.push(c);
            }
        }
        reduced.len()
    }

    fn reduced_len_without(&self, exclude: char) -> usize {
        let mut reduced = Vec::new();
        for c in self.0.chars() {
            if c.to_ascii_lowercase() == exclude {
                continue;
            } else if char_reduces_with(c, reduced.last()) {
                reduced.pop();
            } else {
                reduced.push(c);
            }
        }
        reduced.len()
    }
}

#[aoc_generator(day5)]
fn input_frequencies(input: &str) -> Polymer {
    Polymer::new(input.trim())
}

#[aoc(day5, part1)]
fn part1(input: &Polymer) -> usize {
    input.reduced_len()
}

#[aoc(day5, part2)]
fn part2(input: &Polymer) -> usize {
    (b'a'..=b'z')
        .map(char::from)
        .map(|c| input.reduced_len_without(c))
        .min()
        .unwrap()
}
