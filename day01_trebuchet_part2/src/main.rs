#[allow(unused_imports)]
use day01_trebuchet_common::{REAL_DATA, SAMPLE_DATA_2};

fn main() {
    let result = do_work(SAMPLE_DATA_2);
    println!("{}", result);
}

fn do_work<const N: usize>(data: [&str; N]) -> u64 {
    let mut result: u64 = 0;
    for line in data {
        result += get_number_from_line(line);
    }

    result
}

struct Number {
    text: &'static str,
    value: u64,
}

const ALLOWED_NUMBERS: [Number; 18] = [
    { Number {text: "one", value: 1} },
    { Number {text: "1", value: 1} },
    { Number {text: "two", value: 2} },
    { Number {text: "2", value: 2} },
    { Number {text: "three", value: 3} },
    { Number {text: "3", value: 3} },
    { Number {text: "four", value: 4} },
    { Number {text: "4", value: 4} },
    { Number {text: "five", value: 5} },
    { Number {text: "5", value: 5} },
    { Number {text: "six", value: 6} },
    { Number {text: "6", value: 6} },
    { Number {text: "seven", value: 7} },
    { Number {text: "7", value: 7} },
    { Number {text: "eight", value: 8} },
    { Number {text: "8", value: 8} },
    { Number {text: "nine", value: 9} },
    { Number {text: "9", value: 9} },
];

fn get_number_from_line(line: &str) -> u64 {
    let reversed_line = line.chars().rev().collect::<String>();

    let mut first_number_position: Option<usize> = None;
    let mut first_number: Option<u64> = None;

    let mut last_number_position: Option<usize> = None;
    let mut last_number: Option<u64> = None;

    for candidate in ALLOWED_NUMBERS {
        // Find the first number going fowards
        let position = line.find(candidate.text);
        if position.is_some() {
            let position = position.unwrap();
            if first_number_position.is_none() || position < first_number_position.unwrap() {
                first_number_position = Some(position);
                first_number = Some(candidate.value);
            }
        }

        // First the last number going backwards
        let reversed_candidate = candidate.text.chars().rev().collect::<String>();
        let position = reversed_line.find(&reversed_candidate);
        if position.is_some() {
            let position = position.unwrap();
            if last_number_position.is_none() || position < last_number_position.unwrap() {
                last_number_position = Some(position);
                last_number = Some(candidate.value);
            }
        }
    }


    first_number.unwrap() * 10 + last_number.unwrap()
}
