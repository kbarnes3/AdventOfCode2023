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
    id: u64,
    pulls: Vec<Pull>,
}

fn do_work<const N: usize>(data: [GameResults; N]) -> u64 {
    let structured_data = make_structured_results(data);
    let max_pull: Pull = Pull {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut result: u64 = 0;

    for game in structured_data {
        let mut valid: bool = true;

        for pull in game.pulls {
            if pull.red > max_pull.red || pull.green > max_pull.green || pull.blue > max_pull.blue {
                valid = false;
                break;
            }
        }

        if valid {
            result += game.id;
        }
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

    StructuredGameResults { id: data.id, pulls }
}

fn make_structured_results<const N: usize>(data: [GameResults; N]) -> [StructuredGameResults; N] {
    let structured_results: [StructuredGameResults; N] =
        array_init(|i| make_structured_result(&data[i]));

    structured_results
}
