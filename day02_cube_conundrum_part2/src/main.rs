use array_init::array_init;
#[allow(unused_imports)]
use day02_cube_conundrum_common::{GameResults, REAL_DATA, SAMPLE_DATA};

fn main() {
    let result = do_work(REAL_DATA);
    println!("{}", result);
}

struct Pull {
    red: u64,
    green: u64,
    blue: u64,
}

struct StructuredGameResults {
    pulls: Vec<Pull>,
}

fn do_work<const N: usize>(data: [GameResults; N]) -> u64 {
    let structured_data = make_structured_results(data);

    let mut result: u64 = 0;

    for game in structured_data {
        let mut min_red: u64 = 0;
        let mut min_green: u64 = 0;
        let mut min_blue: u64 = 0;

        for pull in game.pulls {
            if pull.red > min_red {
                min_red = pull.red;
            }
            if pull.green > min_green {
                min_green = pull.green;
            }
            if pull.blue > min_blue {
                min_blue = pull.blue;
            }
        }

        let power: u64 = min_red * min_green * min_blue;
        result += power;
    }

    result
}

fn make_structured_result(data: &GameResults) -> StructuredGameResults {
    let mut pulls = Vec::new();
    let pull_strings: Vec<&str> = data.pulls.split("; ").collect();

    for pull_string in pull_strings {
        let mut red: u64 = 0;
        let mut green: u64 = 0;
        let mut blue: u64 = 0;

        let color_strings: Vec<&str> = pull_string.split(", ").collect();

        for color_string in color_strings {
            let parts: Vec<&str> = color_string.split(' ').collect();
            if parts.len() != 2 {
                panic!("Invalid color string: {}", color_string);
            }

            let number: u64 = parts[0].parse().unwrap();
            let color: &str = parts[1];

            match color {
                "red" => {
                    if red != 0 {
                        panic!("Duplicate red: {}", color_string);
                    }
                    red = number
                }
                "green" => {
                    if green != 0 {
                        panic!("Duplicate green: {}", color_string);
                    }
                    green = number
                }
                "blue" => {
                    if blue != 0 {
                        panic!("Duplicate blue: {}", color_string);
                    }
                    blue = number
                }
                _ => panic!("Invalid color: {}", color),
            }
        }

        pulls.push(Pull { red, green, blue });
    }

    StructuredGameResults { pulls }
}

fn make_structured_results<const N: usize>(data: [GameResults; N]) -> [StructuredGameResults; N] {
    let structured_results: [StructuredGameResults; N] =
        array_init(|i| make_structured_result(&data[i]));

    structured_results
}
