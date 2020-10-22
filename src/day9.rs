use aoc_runner_derive::*;

use lazy_static::*;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub players: usize,
    pub last_marble_score: u32,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Input {
    lazy_static! {
        static ref INPUT_REGEX: Regex =
            Regex::new(r"(.+) players; last marble is worth (.+) points")
                .expect("could not unwrap regex");
    }
    let cap = INPUT_REGEX.captures(input).expect("no match");
    Input {
        players: cap[1].parse().expect("could not parse"),
        last_marble_score: cap[2].parse().expect("could not parse"),
    }
}

#[test]
fn test_input_generator() {
    assert_eq!(
        input_generator("439 players; last marble is worth 71307 points\n"),
        Input {
            players: 439,
            last_marble_score: 71307
        }
    );
}

pub struct RingEntry<T> {
    previous: usize,
    next: usize,
    value: T,
}

pub struct Ring<T> {
    entries: Vec<Option<RingEntry<T>>>,
    empty: Vec<usize>,
    current_index: Option<usize>,
}

impl<T> Ring<T> {
    pub fn current_value(&self) -> Option<&T> {
        if let Some(current_index) = self.current_index {
            Some(&self.entries[current_index].as_ref().unwrap().value)
        } else {
            None
        }
    }

    pub fn move_forwards(&mut self) {
        let current_index = self.current_index.unwrap();
        self.current_index = Some(self.entries[current_index].as_ref().unwrap().next)
    }

    pub fn move_backwards(&mut self) {
        let current_index = self.current_index.unwrap();
        self.current_index = Some(self.entries[current_index].as_ref().unwrap().previous)
    }

    pub fn move_n(&mut self, n: isize) {
        if n > 0 {
            for _ in 0..n {
                self.move_forwards()
            }
        } else if n < 0 {
            for _ in 0..(-n) {
                self.move_backwards()
            }
        }
    }

    pub fn remove(&mut self) -> T {
        let current_index = self.current_index.expect("remove on empty ring");
        let entry = self.entries[current_index].take().expect("no entry");
        self.empty.push(current_index);
        if let Some(previous_entry) = self.entries[entry.previous] {
            previous_entry.next = entry.next;
        }
        if let Some(next_entry) = self.entries[entry.next] {
            previous_entry.previous = entry.previous;
        }
        self.current_index = Some(entry.next);
        entry.value
    }

    pub fn add(&mut self, value: T) {
        let location_to_insert = self.empty.pop();
        if let Some(current_index) = self.current_index {
            let old_entry = self.entries[current_index].unwrap();
            let entry = RingEntry {
                previous: current_index,
                next: old_entry.next,
                value
            };
        } else {
            let entry = RingEntry
        }
    }
}

#[derive(Default, Debug)]
struct Board {
    board: Vec<u32>,
    next_marble_to_place: u32,
    last_placed_position: Option<usize>,
    last_marble_worth: u32,
}

impl Board {
    fn place_next(&mut self) -> u32 {
        if self.next_marble_to_place > 0 && self.next_marble_to_place % 23 == 0 {
            let current_marble_score = self.next_marble_to_place;
            let position_to_remove_from = (self
                .last_placed_position
                .expect("multiple of 23 without last_placed_position")
                + self.board.len()
                - 7)
                % self.board.len();
            let removed_marble = self.board.remove(position_to_remove_from);
            self.last_marble_worth = self.next_marble_to_place;
            self.next_marble_to_place += 1;
            self.last_placed_position = Some(position_to_remove_from);
            current_marble_score + removed_marble
        } else {
            let position_to_place = if let Some(last_placed_position) = self.last_placed_position {
                ((last_placed_position + 1) % self.board.len()) + 1
            } else {
                0
            };
            self.board
                .insert(position_to_place, self.next_marble_to_place);
            self.last_marble_worth = self.next_marble_to_place;
            self.next_marble_to_place += 1;
            self.last_placed_position = Some(position_to_place);
            0
        }
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    let mut scores: Vec<u32> = (0..input.players).map(|_| 0).collect();
    let mut current_player = 0usize;
    let mut board = Board::default();
    while board.last_marble_worth != input.last_marble_score {
        let score = board.place_next();
        scores[current_player] += score;
        current_player = (current_player + 1) % input.players;
    }
    *scores.iter().max().expect("no score")
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    solve_part1(&Input {
        players: input.players,
        last_marble_score: input.last_marble_score * 100,
    })
}

#[test]
fn test_solve_part1() {
    assert_eq!(
        solve_part1(&Input {
            players: 10,
            last_marble_score: 1618
        }),
        8317
    );
    assert_eq!(
        solve_part1(&Input {
            players: 13,
            last_marble_score: 7999
        }),
        146373
    );
    assert_eq!(
        solve_part1(&Input {
            players: 17,
            last_marble_score: 1104
        }),
        2764
    );
    assert_eq!(
        solve_part1(&Input {
            players: 21,
            last_marble_score: 6111
        }),
        54718
    );
    assert_eq!(
        solve_part1(&Input {
            players: 30,
            last_marble_score: 5807
        }),
        37305
    );
}
