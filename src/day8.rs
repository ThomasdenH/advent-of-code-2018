use aoc_runner_derive::*;

#[aoc_generator(day8)]
fn input_frequencies(input: &str) -> Vec<u32> {
    input
        .split(" ")
        .map(|i| i.parse::<u32>().unwrap())
        .collect()
}

struct Node {
    child_nodes: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn read_node<'a>(iter: &mut impl Iterator<Item = u32>) -> Node {
        let child_count = iter.next().unwrap();
        let metadata_count = iter.next().unwrap();
        let child_nodes = (0..child_count).map(|_| Node::read_node(iter)).collect();
        let metadata = (0..metadata_count).map(|_| iter.next().unwrap()).collect();
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
fn test_value() {
    let input = input_frequencies("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    assert_eq!(Node::read_node(&mut input.iter().map(|i| *i)).value(), 66)
}

#[aoc(day8, part1)]
fn sum(input: &Vec<u32>) -> u32 {
    Node::read_node(&mut input.iter().map(|i| *i)).sum_metadata()
}

#[aoc(day8, part2)]
fn value(input: &Vec<u32>) -> u32 {
    Node::read_node(&mut input.iter().map(|i| *i)).value()
}
