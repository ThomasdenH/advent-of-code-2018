use aoc_runner_derive::*;

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<u32> {
    input
        .split(' ')
        .map(|i| i.trim().parse::<u32>().unwrap())
        .collect()
}

struct Node {
    child_nodes: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn read_node(iter: &mut Iterator<Item = &u32>) -> Node {
        let child_count = *iter.next().unwrap();
        let metadata_count = *iter.next().unwrap();
        let child_nodes = (0..child_count).map(|_| Node::read_node(iter)).collect();
        let metadata = (0..metadata_count).map(|_| *iter.next().unwrap()).collect();
        Node {
            child_nodes,
            metadata,
        }
    }

    fn sum_metadata(&self) -> u32 {
        self.metadata.iter().sum::<u32>()
            + self.child_nodes.iter().map(Node::sum_metadata).sum::<u32>()
    }

    fn value(&self) -> u32 {
        if self.child_nodes.is_empty() {
            self.metadata.iter().sum::<u32>()
        } else {
            self.metadata
                .iter()
                .filter_map(|md_entry| md_entry.checked_sub(1)) // 0 => None, i => i - 1
                .filter_map(|md_entry| self.child_nodes.get(md_entry as usize))
                .map(Node::value)
                .sum::<u32>()
        }
    }
}

#[test]
fn test_sum_sum_metadata() {
    let input = generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    assert_eq!(Node::read_node(&mut input.iter()).sum_metadata(), 138)
}

#[test]
fn test_value() {
    let input = generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    assert_eq!(Node::read_node(&mut input.iter()).value(), 66)
}

#[aoc(day8, part1)]
fn part1(input: &[u32]) -> u32 {
    Node::read_node(&mut input.iter()).sum_metadata()
}

#[aoc(day8, part2)]
fn part2(input: &[u32]) -> u32 {
    Node::read_node(&mut input.iter()).value()
}

#[test]
fn test_part1() {
    let input_string = crate::util::read_file_to_string("./input/2018/day8.txt");
    let input = generator(&input_string);
    let result = part1(&input);
    assert_eq!(result, 41028);
}

#[test]
fn test_part2() {
    let input_string = crate::util::read_file_to_string("./input/2018/day8.txt");
    let input = generator(&input_string);
    let result = part2(&input);
    assert_eq!(result, 20849);
}
