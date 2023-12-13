#[allow(unused_imports)]
use day08_haunted_wasteland_common::{
    Direction, Map, NodeLink, END_SUFFIX, REAL_DATA, SAMPLE_DATA_3, START_SUFFIX,
};
use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    let result = do_work(REAL_DATA);
    println!("{}", result);
}

fn do_work<const D: usize, const N: usize>(data: Map<D, N>) -> u64 {
    let links: HashMap<&str, Link> = make_link_map(&data.nodes);
    find_ends(&data.directions, &links)
}

struct Link<'a> {
    left: &'a str,
    right: &'a str,
}

fn make_link_map<'a, const N: usize>(nodes: &[NodeLink<'a>; N]) -> HashMap<&'a str, Link<'a>> {
    let mut links: HashMap<&'a str, Link<'a>> = HashMap::new();
    for node in nodes {
        links.insert(
            node.node,
            Link {
                left: node.left,
                right: node.right,
            },
        );
    }
    links
}

fn get_start_nodes<'a>(links: &HashMap<&'a str, Link>) -> Vec<&'a str> {
    let mut start_nodes: Vec<&str> = Vec::new();
    for node in links.keys() {
        if node.ends_with(START_SUFFIX) {
            start_nodes.push(node);
        }
    }
    start_nodes
}

fn is_node_done(node: &str) -> bool {
    node.ends_with(END_SUFFIX)
}

fn find_path_length_to_end<const D: usize>(node: &str, directions: &[Direction; D], links: &HashMap<&str, Link>) -> u64 {
    let mut current_node = node;
    let mut distance: u64 = 0;

    loop {
        for direction in directions {
            if is_node_done(current_node) {
                return distance;
            }

            current_node = match direction {
                Direction::Left => links.get(current_node).unwrap().left,
                Direction::Right => links.get(current_node).unwrap().right,
            };

            distance += 1;
        }
    }
}

fn find_ends<const D: usize>(directions: &[Direction; D], links: &HashMap<&str, Link>) -> u64 {
    let start_nodes: Vec<&str> = get_start_nodes(links);
    let mut loop_lengths: Vec<u64> = Vec::with_capacity(start_nodes.len());
    
    for node in start_nodes {
        loop_lengths.push(find_path_length_to_end(node, directions, links));
    }

    let mut total_length: u64 = 1;
    for length in loop_lengths {
        total_length = lcm(total_length, length);
    }

    total_length
}
