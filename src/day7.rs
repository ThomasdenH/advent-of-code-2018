use aoc_runner_derive::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref EDGE: Regex =
        Regex::new(r"Step (.+) must be finished before step (.+) can begin.").unwrap();
}

#[aoc_generator(day7)]
fn input_frequencies(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            let cap = EDGE.captures(line).unwrap();
            (
                cap[1].chars().next().unwrap(),
                cap[2].chars().next().unwrap(),
            )
        })
        .collect()
}

struct Graph {
    // Maps a node to the nodes preceding it.
    waiting_on: HashMap<char, HashSet<char>>,
    nodes: HashSet<char>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            waiting_on: HashMap::new(),
            nodes: HashSet::new(),
        }
    }

    fn add_edge(&mut self, from: char, to: char) {
        self.nodes.insert(from);
        self.nodes.insert(to);
        let mut current_map = self
            .waiting_on
            .remove(&from)
            .unwrap_or_else(|| HashSet::new());
        current_map.insert(to);
        self.waiting_on.insert(from, current_map);
    }

    fn remove_first(&mut self) -> char {
        // Find the alphabetically first item without parents
        let to_remove = *self
            .nodes
            .iter()
            .filter(|value| {
                self.waiting_on
                    .get(value)
                    .map(HashSet::is_empty)
                    .unwrap_or(true)
            })
            .min()
            .unwrap();

        // Remove from set
        self.nodes.remove(&to_remove);
        self.waiting_on.remove(&to_remove);

        // Remove from others' waiting list
        for list in self.waiting_on.values_mut() {
            list.remove(&to_remove);
        }

        to_remove
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[aoc(day7, part1)]
fn part1(input: &Vec<(char, char)>) -> String {
    let mut graph = Graph::new();
    for (to, from) in input {
        graph.add_edge(*from, *to);
    }

    (0..graph.len()).map(|_| graph.remove_first()).collect()
}
