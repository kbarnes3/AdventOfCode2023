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

pub const REAL_DATA: [GameResults; 100] = [
    {
        GameResults { id: 1, pulls: "4 red, 18 green, 15 blue; 17 green, 18 blue, 9 red; 8 red, 14 green, 6 blue; 14 green, 12 blue, 2 red" }
    },
    {
        GameResults {
            id: 2,
            pulls: "6 red, 11 green; 4 blue, 4 green, 5 red; 11 green, 6 blue, 6 red",
        }
    },
    {
        GameResults { id: 3, pulls: "3 red, 3 green; 3 green, 1 blue, 7 red; 3 green, 5 red, 1 blue; 9 red, 4 green; 1 blue, 2 green, 5 red" }
    },
    {
        GameResults { id: 4, pulls: "2 blue, 5 green, 9 red; 7 red, 10 blue; 2 green, 14 blue, 5 red; 3 blue, 2 green; 4 green, 10 red, 7 blue; 2 green, 15 blue, 7 red" }
    },
    {
        GameResults {
            id: 5,
            pulls: "3 red, 2 blue; 5 red, 3 blue; 10 blue, 10 red, 1 green; 4 blue",
        }
    },
    {
        GameResults { id: 6, pulls: "1 green, 10 blue, 5 red; 8 blue, 9 green; 20 green, 7 red, 10 blue; 12 green, 6 blue, 6 red; 10 blue, 11 green; 8 blue, 17 green, 5 red" }
    },
    {
        GameResults { id: 7, pulls: "7 green, 12 blue, 3 red; 19 red, 12 blue; 8 blue, 8 red, 7 green; 6 red, 7 green, 5 blue" }
    },
    {
        GameResults { id: 8, pulls: "8 blue, 7 red; 13 green, 5 blue, 5 red; 11 blue, 4 green, 7 red; 5 blue, 6 red, 13 green; 7 blue, 12 green, 8 red" }
    },
    {
        GameResults { id: 9, pulls: "3 red, 3 blue, 12 green; 2 red, 1 blue, 9 green; 3 red, 12 green, 3 blue; 2 red, 7 green, 2 blue; 8 green, 4 blue; 2 red, 2 green" }
    },
    {
        GameResults {
            id: 10,
            pulls: "16 green, 10 red; 13 green, 7 red; 8 red, 1 blue, 8 green",
        }
    },
    {
        GameResults {
            id: 11,
            pulls: "7 red, 1 blue, 1 green; 6 blue, 1 green, 3 red; 5 blue, 10 red",
        }
    },
    {
        GameResults { id: 12, pulls: "1 green, 8 red, 5 blue; 6 red, 12 blue; 2 blue, 15 red; 14 blue, 15 red, 1 green; 8 red, 9 blue" }
    },
    {
        GameResults {
            id: 13,
            pulls: "1 green, 6 red; 7 blue, 13 red, 1 green; 3 blue, 4 red",
        }
    },
    {
        GameResults {
            id: 14,
            pulls: "11 red, 1 green, 1 blue; 3 blue, 18 red, 15 green; 10 blue, 5 green, 11 red",
        }
    },
    {
        GameResults { id: 15, pulls: "6 green, 10 blue, 15 red; 6 green, 17 blue, 8 red; 19 red, 7 blue, 2 green; 1 green, 18 red, 4 blue" }
    },
    {
        GameResults {
            id: 16,
            pulls: "1 green, 17 red, 7 blue; 12 red, 10 green, 9 blue; 15 red, 3 green, 15 blue",
        }
    },
    {
        GameResults {
            id: 17,
            pulls: "12 blue, 13 green; 16 green, 19 blue, 7 red; 1 green, 2 blue",
        }
    },
    {
        GameResults { id: 18, pulls: "8 blue, 9 green, 2 red; 9 blue, 7 green; 3 red, 9 green, 10 blue; 1 blue, 7 green, 2 red; 1 green, 8 blue, 4 red" }
    },
    {
        GameResults {
            id: 19,
            pulls: "3 green, 2 red, 11 blue; 13 blue, 3 green, 1 red; 1 red, 10 blue",
        }
    },
    {
        GameResults {
            id: 20,
            pulls:
                "2 red, 4 green, 1 blue; 14 blue, 7 green; 7 blue, 9 green; 4 red, 5 green, 7 blue",
        }
    },
    {
        GameResults { id: 21, pulls: "4 blue, 20 red, 7 green; 4 green, 6 blue, 14 red; 6 green, 18 red, 5 blue; 2 blue, 4 green, 6 red; 4 green, 16 red, 4 blue" }
    },
    {
        GameResults {
            id: 22,
            pulls: "13 red, 2 green; 6 red, 3 blue; 6 red, 2 green; 7 red, 1 green; 6 red, 2 green",
        }
    },
    {
        GameResults {
            id: 23,
            pulls:
                "5 blue; 6 red, 16 green, 12 blue; 1 blue, 6 green, 2 red; 8 red, 6 blue, 3 green",
        }
    },
    {
        GameResults { id: 24, pulls: "10 green, 4 blue, 5 red; 1 green, 3 red; 8 red, 3 blue, 6 green; 3 red, 2 blue; 3 red, 10 green, 3 blue" }
    },
    {
        GameResults {
            id: 25,
            pulls: "1 red, 2 green; 4 green, 6 red, 1 blue; 3 red; 4 green, 2 red",
        }
    },
    {
        GameResults { id: 26, pulls: "7 red, 1 blue; 2 red, 1 blue; 9 red, 1 green, 2 blue; 5 red, 2 blue; 4 red, 2 green; 8 red, 1 green, 2 blue" }
    },
    {
        GameResults { id: 27, pulls: "1 green, 2 red, 8 blue; 1 green, 4 red, 9 blue; 16 blue, 12 red, 3 green; 13 blue, 4 green, 5 red" }
    },
    {
        GameResults {
            id: 28,
            pulls: "8 blue, 8 green, 3 red; 8 green, 6 blue; 5 green, 6 blue, 4 red",
        }
    },
    {
        GameResults { id: 29, pulls: "7 red, 11 green, 5 blue; 1 green, 1 blue, 6 red; 6 green, 5 blue, 8 red; 7 blue, 15 green, 2 red; 10 blue, 1 red" }
    },
    {
        GameResults { id: 30, pulls: "7 red, 5 blue, 14 green; 2 blue, 11 red; 17 green, 2 blue, 7 red; 4 blue, 10 red, 5 green" }
    },
    {
        GameResults { id: 31, pulls: "17 blue, 5 red, 2 green; 7 red, 14 blue, 3 green; 13 blue, 5 red, 2 green; 12 green, 8 blue, 8 red" }
    },
    {
        GameResults {
            id: 32,
            pulls: "1 red, 7 blue; 1 red, 8 blue; 1 red, 2 green, 13 blue",
        }
    },
    {
        GameResults { id: 33, pulls: "1 green, 3 blue, 3 red; 4 red, 2 green; 5 blue, 1 red, 1 green; 1 red, 8 blue, 2 green" }
    },
    {
        GameResults {
            id: 34,
            pulls: "9 blue, 7 red; 9 green, 11 red, 1 blue; 18 red, 4 blue, 6 green",
        }
    },
    {
        GameResults { id: 35, pulls: "7 blue, 4 green, 2 red; 1 green, 1 blue, 2 red; 3 green; 3 blue, 7 green, 1 red; 7 blue, 12 green" }
    },
    {
        GameResults { id: 36, pulls: "17 red, 5 blue; 6 red, 5 green, 7 blue; 16 blue, 1 green, 7 red; 7 blue, 5 green, 15 red; 8 blue, 19 red, 1 green" }
    },
    {
        GameResults { id: 37, pulls: "4 blue, 6 red, 1 green; 9 red, 8 green, 4 blue; 1 green, 8 blue, 10 red; 11 green, 6 red, 9 blue" }
    },
    {
        GameResults { id: 38, pulls: "3 red, 4 blue; 5 red, 1 blue; 1 green, 2 red, 5 blue; 2 blue, 8 red; 7 red, 1 blue; 4 blue, 5 red" }
    },
    {
        GameResults {
            id: 39,
            pulls: "7 green; 5 green; 3 blue; 12 green, 1 red, 1 blue; 8 green, 1 blue, 1 red",
        }
    },
    {
        GameResults {
            id: 40,
            pulls: "12 red, 11 blue; 6 green, 2 blue, 13 red; 6 green, 7 red, 6 blue",
        }
    },
    {
        GameResults { id: 41, pulls: "3 green, 1 blue; 5 blue, 7 red, 6 green; 6 red, 14 blue; 9 red, 14 green, 5 blue; 5 blue, 6 green, 3 red; 20 green, 4 blue, 5 red" }
    },
    {
        GameResults {
            id: 42,
            pulls: "2 blue, 13 green; 10 red, 6 green; 8 green, 2 red; 7 red",
        }
    },
    {
        GameResults { id: 43, pulls: "7 green, 3 red; 6 red, 6 green, 13 blue; 7 green, 2 red, 9 blue; 8 blue, 3 green, 1 red; 10 green, 7 red, 13 blue" }
    },
    {
        GameResults {
            id: 44,
            pulls: "3 blue, 1 green, 2 red; 10 blue, 9 red; 5 red, 13 blue",
        }
    },
    {
        GameResults { id: 45, pulls: "11 red, 2 green, 5 blue; 1 green, 6 red, 6 blue; 17 red, 2 green, 6 blue; 14 red, 2 green" }
    },
    {
        GameResults {
            id: 46,
            pulls: "5 blue, 7 red, 8 green; 6 green, 1 red, 10 blue; 1 red, 5 blue, 4 green",
        }
    },
    {
        GameResults { id: 47, pulls: "5 green, 5 red, 1 blue; 11 green, 8 red, 6 blue; 2 green, 16 red, 1 blue; 12 green, 1 red, 7 blue; 2 red, 15 green, 7 blue" }
    },
    {
        GameResults {
            id: 48,
            pulls: "3 red, 6 green, 4 blue; 1 blue, 1 green, 2 red; 12 blue, 7 green, 5 red",
        }
    },
    {
        GameResults {
            id: 49,
            pulls: "4 blue, 1 green; 4 red, 2 blue; 3 blue, 2 green; 5 red, 3 blue, 4 green",
        }
    },
    {
        GameResults {
            id: 50,
            pulls: "1 blue, 1 green; 3 blue, 7 red, 1 green; 2 blue, 1 green",
        }
    },
    {
        GameResults {
            id: 51,
            pulls: "17 blue, 1 green, 3 red; 2 green, 1 red, 3 blue; 14 blue, 10 red",
        }
    },
    {
        GameResults {
            id: 52,
            pulls: "8 blue, 1 green; 1 blue, 3 red, 2 green; 2 green, 14 blue",
        }
    },
    {
        GameResults { id: 53, pulls: "9 green, 3 blue, 9 red; 3 blue, 7 red, 8 green; 2 green, 2 red; 17 green, 3 red; 18 green, 8 red" }
    },
    {
        GameResults {
            id: 54,
            pulls: "2 blue, 10 red; 2 green, 2 red; 6 green, 1 blue, 1 red; 3 blue, 6 red, 7 green",
        }
    },
    {
        GameResults { id: 55, pulls: "3 blue, 1 red; 1 green, 2 red, 1 blue; 4 blue, 3 red; 5 blue, 3 green; 3 green, 1 red, 3 blue; 2 green" }
    },
    {
        GameResults { id: 56, pulls: "10 green, 1 red, 6 blue; 16 green, 1 blue, 10 red; 8 red, 9 green, 2 blue; 3 red, 2 blue" }
    },
    {
        GameResults {
            id: 57,
            pulls: "1 blue, 4 green, 1 red; 7 red, 4 green, 8 blue; 9 red, 3 blue, 3 green",
        }
    },
    {
        GameResults { id: 58, pulls: "15 green, 16 blue, 8 red; 8 blue, 8 red, 2 green; 9 blue, 8 red, 3 green; 20 blue, 15 green, 7 red" }
    },
    {
        GameResults {
            id: 59,
            pulls:
                "13 red, 3 blue; 12 red, 4 green; 9 blue, 5 green, 9 red; 2 red, 12 blue, 1 green",
        }
    },
    {
        GameResults { id: 60, pulls: "14 green, 16 red; 5 green, 1 blue, 5 red; 14 green, 5 blue, 20 red; 2 blue, 8 green, 1 red" }
    },
    {
        GameResults { id: 61, pulls: "2 green, 10 red, 15 blue; 17 blue, 6 red, 2 green; 2 red, 2 green, 12 blue; 2 red, 2 green" }
    },
    {
        GameResults { id: 62, pulls: "8 blue, 1 green, 3 red; 6 red, 15 blue, 2 green; 5 green, 6 blue; 1 red, 7 green, 8 blue" }
    },
    {
        GameResults {
            id: 63,
            pulls: "13 green, 8 red; 8 green, 1 blue, 5 red; 2 green, 8 red, 2 blue",
        }
    },
    {
        GameResults { id: 64, pulls: "13 red, 12 blue, 4 green; 2 blue, 3 red, 1 green; 4 green, 14 red, 14 blue; 8 red, 4 green; 16 red; 5 blue, 16 red, 4 green" }
    },
    {
        GameResults {
            id: 65,
            pulls: "13 red, 2 blue, 3 green; 10 red, 6 blue; 6 blue, 5 red",
        }
    },
    {
        GameResults { id: 66, pulls: "1 blue, 9 green; 4 green, 5 blue; 8 green, 8 blue; 10 blue, 1 red, 10 green; 18 blue, 1 red, 9 green" }
    },
    {
        GameResults {
            id: 67,
            pulls: "12 red, 7 blue; 13 red, 3 blue, 3 green; 7 blue, 6 red, 4 green",
        }
    },
    {
        GameResults { id: 68, pulls: "3 green, 4 blue, 8 red; 1 green, 2 blue, 13 red; 3 green, 14 red, 4 blue; 6 red, 4 green; 7 blue, 2 red, 1 green; 1 green, 3 blue, 14 red" }
    },
    {
        GameResults { id: 69, pulls: "2 blue, 6 red, 2 green; 7 green, 18 red; 11 green, 1 blue, 13 red; 3 red, 6 green, 1 blue; 19 red, 1 green" }
    },
    {
        GameResults { id: 70, pulls: "13 green; 1 red, 14 green, 2 blue; 9 red, 1 blue, 9 green; 6 green, 5 red, 1 blue; 2 green, 10 red" }
    },
    {
        GameResults { id: 71, pulls: "7 blue, 5 green, 11 red; 4 red, 8 blue, 5 green; 1 green, 1 blue; 6 green, 8 red, 5 blue; 8 red, 7 green, 6 blue" }
    },
    {
        GameResults {
            id: 72,
            pulls: "2 blue, 2 green, 1 red; 5 green, 1 red, 3 blue; 4 green",
        }
    },
    {
        GameResults { id: 73, pulls: "8 green, 3 blue, 3 red; 1 green, 3 red, 9 blue; 3 red, 10 blue, 8 green; 10 green, 3 red, 8 blue; 3 blue, 3 green; 2 green" }
    },
    {
        GameResults {
            id: 74,
            pulls: "5 red, 1 green; 1 blue, 5 red; 8 red, 3 blue",
        }
    },
    {
        GameResults { id: 75, pulls: "5 red, 7 green, 3 blue; 1 red, 5 blue, 4 green; 2 blue, 12 green; 3 blue, 5 red; 8 green, 4 blue, 3 red; 1 green, 2 blue, 1 red" }
    },
    {
        GameResults { id: 76, pulls: "10 green, 5 blue, 1 red; 11 blue, 16 green, 1 red; 12 blue, 2 red, 18 green; 12 green, 10 blue; 5 blue, 5 green, 1 red; 9 green, 1 red, 1 blue" }
    },
    {
        GameResults {
            id: 77,
            pulls: "9 blue, 1 red, 2 green; 1 blue, 1 red, 5 green; 5 blue",
        }
    },
    {
        GameResults {
            id: 78,
            pulls: "1 red, 1 blue; 1 blue; 1 red; 1 green, 2 red, 1 blue; 1 blue, 4 red",
        }
    },
    {
        GameResults { id: 79, pulls: "3 green, 11 red, 4 blue; 7 red, 1 green, 4 blue; 1 green, 3 red, 3 blue; 3 blue, 3 red, 4 green; 3 green, 3 blue, 9 red" }
    },
    {
        GameResults {
            id: 80,
            pulls: "11 blue, 10 green, 11 red; 10 green, 9 red, 18 blue; 11 green, 17 blue, 7 red",
        }
    },
    {
        GameResults { id: 81, pulls: "6 red, 1 blue; 3 blue, 6 red, 2 green; 6 red, 10 green, 1 blue; 5 blue, 3 green, 3 red" }
    },
    {
        GameResults { id: 82, pulls: "6 red, 16 green, 2 blue; 9 green, 6 red, 3 blue; 1 blue, 9 red, 14 green; 8 green, 11 red, 3 blue; 3 red, 5 green; 12 green, 3 blue" }
    },
    {
        GameResults { id: 83, pulls: "7 blue, 5 green, 11 red; 8 red, 9 blue, 13 green; 13 blue, 8 red, 8 green; 2 blue, 9 green, 5 red" }
    },
    {
        GameResults { id: 84, pulls: "9 green, 14 red, 11 blue; 1 green, 12 blue, 6 red; 12 green, 10 red, 7 blue; 15 green, 6 blue; 15 blue, 4 red, 6 green; 16 green, 2 red, 13 blue" }
    },
    {
        GameResults {
            id: 85,
            pulls: "7 red, 7 blue, 3 green; 5 green, 1 blue; 6 red, 11 green, 7 blue",
        }
    },
    {
        GameResults { id: 86, pulls: "9 green, 6 blue, 6 red; 3 red, 2 blue, 7 green; 4 red, 4 green, 7 blue; 10 blue, 10 green, 2 red; 5 green" }
    },
    {
        GameResults {
            id: 87,
            pulls: "6 green, 5 blue; 15 blue, 9 green, 1 red; 14 blue, 15 green",
        }
    },
    {
        GameResults { id: 88, pulls: "3 blue, 2 green, 5 red; 8 blue, 1 green, 2 red; 5 red, 8 blue, 1 green; 1 red, 1 blue; 1 green, 6 red, 2 blue; 1 green, 2 red, 1 blue" }
    },
    {
        GameResults {
            id: 89,
            pulls: "4 blue, 3 green; 1 blue, 2 red; 2 red, 1 green, 4 blue; 2 red, 2 blue, 1 green",
        }
    },
    {
        GameResults {
            id: 90,
            pulls: "2 green, 1 red; 3 green, 8 red; 1 blue, 6 red, 4 green",
        }
    },
    {
        GameResults {
            id: 91,
            pulls: "3 red; 1 blue, 6 red; 1 blue, 5 red, 1 green",
        }
    },
    {
        GameResults {
            id: 92,
            pulls: "6 red, 9 green, 7 blue; 9 green, 4 red; 2 green, 5 blue",
        }
    },
    {
        GameResults { id: 93, pulls: "7 green, 1 red; 3 blue, 3 red; 3 green, 9 red, 4 blue; 2 red, 6 green; 5 red, 3 blue" }
    },
    {
        GameResults { id: 94, pulls: "4 green, 11 red; 13 green, 9 red; 16 green, 11 red; 6 green, 2 blue, 14 red; 17 green, 9 red" }
    },
    {
        GameResults { id: 95, pulls: "7 red, 13 blue, 2 green; 8 green, 13 blue, 3 red; 5 green, 6 red, 13 blue; 8 green, 8 blue, 2 red; 6 blue, 4 green, 8 red; 2 blue, 2 red" }
    },
    {
        GameResults { id: 96, pulls: "10 red, 3 blue, 3 green; 2 blue, 4 green, 5 red; 7 blue, 4 green, 6 red; 1 green, 4 red, 5 blue" }
    },
    {
        GameResults {
            id: 97,
            pulls: "5 red, 8 blue; 4 green, 2 red, 14 blue; 10 blue, 7 green",
        }
    },
    {
        GameResults { id: 98, pulls: "1 red, 2 green, 14 blue; 6 green, 1 blue; 19 blue, 4 red; 18 blue, 4 red, 3 green; 2 red, 1 blue" }
    },
    {
        GameResults { id: 99, pulls: "3 red, 4 blue; 7 red, 5 blue, 3 green; 2 green, 1 blue, 1 red; 4 blue, 2 green, 1 red; 1 green, 1 red, 2 blue; 1 green, 6 blue, 7 red" }
    },
    {
        GameResults { id: 100, pulls: "2 blue, 10 green; 10 green, 14 red; 3 green, 5 red, 2 blue; 1 red, 3 blue, 7 green; 1 blue, 7 red" }
    },
];
