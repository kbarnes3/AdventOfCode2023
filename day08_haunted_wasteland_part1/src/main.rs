#[allow(unused_imports)]
use day08_haunted_wasteland_common::{Direction, END_NODE, Map, NodeLink, SAMPLE_DATA, START_NODE};
use std::collections::HashMap;

fn main() {
    let result = do_work(SAMPLE_DATA);
    println!("{}", result);
}

fn do_work<const D: usize, const N: usize>(data: Map<D, N>) -> u64 {
    let links: HashMap<&str, Link> = make_link_map(&data.nodes);
    find_end(&data.directions, &links)
}

struct Link<'a> {
    left: &'a str,
    right: &'a str,
}

fn make_link_map<'a, const N: usize>(nodes: &[NodeLink<'a>; N]) -> HashMap<&'a str, Link<'a>> {
    let mut links: HashMap<&'a str, Link<'a>> = HashMap::new();
    for node in nodes {
        links.insert(node.node, Link { left: node.left, right: node.right });
    }
    links
}

fn find_end<const D: usize>(directions: &[Direction; D], links: &HashMap<&str, Link>) -> u64 {
    let mut current_node: &str = START_NODE;
    let mut distance: u64 = 0;

    loop {
        for direction in directions {
            if current_node == END_NODE {
                return distance;
            }

            let link: &Link = links.get(current_node).unwrap();
            match direction {
                Direction::Left => {
                    current_node = link.left;
                }
                Direction::Right => {
                    current_node = link.right;
                }
            }
            distance += 1;
        }
    }
}
