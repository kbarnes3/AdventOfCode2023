#[allow(unused_imports)]
use day01_trebuchet_common::{REAL_DATA, SAMPLE_DATA};

fn main() {
    let result = do_work(REAL_DATA);
    println!("{}", result);
}

fn do_work<const N: usize>(data: [&str; N]) -> u64 {
    let mut result: u64 = 0;
    for line in data {
        result += get_number_from_line(line);
    }

    result
}

fn get_number_from_line(line: &str) -> u64 {
    let first_number: u64 = line
        .chars()
        .find(|c| c.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap() as u64;
    let last_number: u64 = line
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap() as u64;

    first_number * 10 + last_number
}
