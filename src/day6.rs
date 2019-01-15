use aoc_runner_derive::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

#[aoc_generator(day6)]
fn generator(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let vec = line
                .split(", ")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (vec[0], vec[1])
        })
        .collect()
}

enum Closest {
    Singular(usize),
    Multiple,
}

impl fmt::Display for Closest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Closest::Singular(c) => write!(f, "{}", c),
            Closest::Multiple => write!(f, "*"),
        }
    }
}

struct AreaPlot {
    closest: Closest,
    distance: u32,
}

enum Area {
    Finite(u32),
    Infinite,
}

struct AreaOfIndexMap(HashMap<usize, Area>);

impl AreaOfIndexMap {
    fn new() -> AreaOfIndexMap {
        AreaOfIndexMap(HashMap::new())
    }

    fn increase_by_one(&mut self, index: usize) {
        match self.0.get(&index) {
            None => {
                self.0.insert(index, Area::Finite(1));
            }
            Some(Area::Finite(current_area)) => {
                self.0.insert(index, Area::Finite(current_area + 1));
            }
            _ => {}
        }
    }

    fn decrease_by_one(&mut self, index: usize) {
        match self.0.get(&index) {
            Some(Area::Finite(current_area)) => {
                self.0.insert(index, Area::Finite(current_area - 1));
            }
            None => panic!("Index {} already has no area", index),
            _ => {}
        }
    }

    fn set_infinite(&mut self, index: usize) {
        self.0.insert(index, Area::Infinite);
    }

    fn max_finite_area(&self) -> u32 {
        *self
            .0
            .values()
            .filter_map(|area| match area {
                Area::Finite(area) => Some(area),
                Area::Infinite => None,
            })
            .max()
            .unwrap()
    }
}

struct QueueEntry {
    index: usize,
    coord: (u32, u32),
    distance: u32,
}

#[aoc(day6, part1)]
fn part1(input: &[(u32, u32)]) -> u32 {
    let min_x = *input.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *input.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *input.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *input.iter().map(|(_, y)| y).max().unwrap();

    let mut map: HashMap<(u32, u32), AreaPlot> = HashMap::new();
    let mut queue = input
        .iter()
        .enumerate()
        .map(|(index, coord)| QueueEntry {
            index,
            coord: *coord,
            distance: 0,
        })
        .collect::<VecDeque<QueueEntry>>();
    let mut area_of_index = AreaOfIndexMap::new();
    while let Some(QueueEntry {
        index,
        coord,
        distance,
    }) = queue.pop_front()
    {
        if coord.0 < min_x || coord.0 > max_x || coord.1 < min_y || coord.1 > max_y {
            // Outside the bounds, so the area is infinite
            area_of_index.set_infinite(index);
        } else {
            match map.get_mut(&coord) {
                None => {
                    // There is not yet a closest plot.
                    // Draw on map...
                    map.insert(
                        coord,
                        AreaPlot {
                            closest: Closest::Singular(index),
                            distance,
                        },
                    );
                    // ... and add to the area
                    area_of_index.increase_by_one(index);

                    // Update the queue
                    queue.push_back(QueueEntry {
                        index,
                        coord: (coord.0 + 1, coord.1),
                        distance: distance + 1,
                    });
                    queue.push_back(QueueEntry {
                        index,
                        coord: (coord.0 - 1, coord.1),
                        distance: distance + 1,
                    });
                    queue.push_back(QueueEntry {
                        index,
                        coord: (coord.0, coord.1 - 1),
                        distance: distance + 1,
                    });
                    queue.push_back(QueueEntry {
                        index,
                        coord: (coord.0, coord.1 + 1),
                        distance: distance + 1,
                    });
                }
                Some(area_plot) => {
                    // There is another plot closer or as close
                    assert!(area_plot.distance <= distance);

                    if area_plot.distance == distance {
                        // The other plot is as close, remove one from its area
                        if let Closest::Singular(closest_index) = area_plot.closest {
                            if closest_index == index {
                                // This index is already closest
                                continue;
                            }
                            area_of_index.decrease_by_one(closest_index);
                        }
                        area_plot.closest = Closest::Multiple;
                    }
                }
            }
        }
    }

    area_of_index.max_finite_area()
}

#[aoc(day6, part2)]
fn part2(input: &[(u32, u32)]) -> usize {
    reagion_area_total_distance_less_than(input, 10_000)
}

fn reagion_area_total_distance_less_than(input: &[(u32, u32)], max_distance: u32) -> usize {
    // An inefficient but short method
    let min_x = i64::from(*input.iter().map(|(a, _)| a).min().unwrap());
    let max_x = i64::from(*input.iter().map(|(a, _)| a).max().unwrap());
    let min_y = i64::from(*input.iter().map(|(_, a)| a).min().unwrap());
    let max_y = i64::from(*input.iter().map(|(_, a)| a).max().unwrap());
    let padding = 1 + (i64::from(max_distance) - 1) / input.len() as i64;

    ((min_x - padding)..(max_x + padding))
        .flat_map(|x| {
            ((min_y - padding)..(max_y + padding)).filter(move |y| {
                input
                    .iter()
                    .map(|coord| distance((x, *y), *coord))
                    .sum::<u32>()
                    < max_distance
            })
        })
        .count()
}

fn distance<T: Into<i64>, Y: Into<i64>>(a: (T, T), b: (Y, Y)) -> u32 {
    let a = (a.0.into(), a.1.into());
    let b = (b.0.into(), b.1.into());
    (a.0 - b.0).abs() as u32 + (a.1 - b.1).abs() as u32
}

#[test]
fn test_part1() {
    let input_string = crate::util::read_file_to_string("./input/2018/day6.txt");
    let input = generator(&input_string);
    let result = part1(&input);
    assert_eq!(result, 4589);
}

#[test]
fn test_part1_example() {
    let input_string = "1, 1\n\
                        1, 6\n\
                        8, 3\n\
                        3, 4\n\
                        5, 5\n\
                        8, 9";
    let input = generator(&input_string);
    let result = part1(&input);
    assert_eq!(result, 17);
}

#[test]
fn test_part2() {
    let input_string = crate::util::read_file_to_string("./input/2018/day6.txt");
    let input = generator(&input_string);
    let result = part2(&input);
    assert_eq!(result, 40252);
}

#[test]
fn test_part2_example() {
    let input_string = "1, 1\n\
                        1, 6\n\
                        8, 3\n\
                        3, 4\n\
                        5, 5\n\
                        8, 9";
    let input = generator(&input_string);
    let result = reagion_area_total_distance_less_than(&input, 32);
    assert_eq!(result, 16);
}
