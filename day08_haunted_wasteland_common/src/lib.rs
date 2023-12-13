pub enum Direction {
    Left,
    Right,
}

pub struct NodeLink<'a> {
    pub node: &'a str,
    pub left: &'a str,
    pub right: &'a str,
}

pub struct Map<'a, const D: usize, const N: usize> {
    pub directions: [Direction; D],
    pub nodes: [NodeLink<'a>; N],
}

pub const START_NODE: &str = "AAA";
pub const END_NODE: &str = "ZZZ";

// Transform with:
// '<,'>s/L/Direction::Left, /g | '<,'>s/R/Direction::Right, /g
// '<,'>s/\(\w\+\) = (\(\w\+\), \(\w\+\))/{ NodeLink { node: "\1", left: "\2", right: "\3" } },

pub const SAMPLE_DATA: Map<2, 7> = Map {
    directions: [Direction::Right, Direction::Left],

    nodes: [
        {
            NodeLink {
                node: "AAA",
                left: "BBB",
                right: "CCC",
            }
        },
        {
            NodeLink {
                node: "BBB",
                left: "DDD",
                right: "EEE",
            }
        },
        {
            NodeLink {
                node: "CCC",
                left: "ZZZ",
                right: "GGG",
            }
        },
        {
            NodeLink {
                node: "DDD",
                left: "DDD",
                right: "DDD",
            }
        },
        {
            NodeLink {
                node: "EEE",
                left: "EEE",
                right: "EEE",
            }
        },
        {
            NodeLink {
                node: "GGG",
                left: "GGG",
                right: "GGG",
            }
        },
        {
            NodeLink {
                node: "ZZZ",
                left: "ZZZ",
                right: "ZZZ",
            }
        },
    ],
};
