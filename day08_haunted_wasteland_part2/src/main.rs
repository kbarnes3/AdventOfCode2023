#[allow(unused_imports)]
use day08_haunted_wasteland_common::{
    Direction, Map, NodeLink, END_SUFFIX, REAL_DATA, SAMPLE_DATA_3, START_SUFFIX,
};
use std::collections::HashMap;

fn main() {
    let result = do_work(SAMPLE_DATA_3);
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

fn are_all_nodes_done(nodes: &Vec<&str>) -> bool {
    for node in nodes {
        if !node.ends_with(END_SUFFIX) {
            return false;
        }
    }
    true
}

fn find_ends<const D: usize>(directions: &[Direction; D], links: &HashMap<&str, Link>) -> u64 {
    let mut current_nodes: Vec<&str> = get_start_nodes(links);
    let mut distance: u64 = 0;

    while !are_all_nodes_done(&current_nodes) {
        for direction in directions {
            for node in &mut current_nodes {
                match direction {
                    Direction::Left => {
                        let link = links.get(node).unwrap();
                        *node = link.left;
                    }
                    Direction::Right => {
                        let link = links.get(node).unwrap();
                        *node = link.right;
                    }
                }
            }
            distance += 1;
        }
    }

    distance
}
