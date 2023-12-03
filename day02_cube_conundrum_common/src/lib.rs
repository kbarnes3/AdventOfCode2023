pub struct GameResults {
    pub id: u64,
    pub pulls: &'static str,
}

pub const SAMPLE_DATA: [GameResults; 5] = [
    {
        GameResults {
            id: 1,
            pulls: "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        }
    },
    {
        GameResults {
            id: 2,
            pulls: "1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        }
    },
    {
        GameResults {
            id: 3,
            pulls: "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        }
    },
    {
        GameResults {
            id: 4,
            pulls: "1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        }
    },
    {
        GameResults {
            id: 5,
            pulls: "6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        }
    },
];
