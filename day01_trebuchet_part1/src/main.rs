use day01_trebuchet_common::SAMPLE_DATA;

fn main() {
    let result = do_work(SAMPLE_DATA);
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
    let first_number: u64 = 
        line.chars().into_iter().find(|c| c.is_numeric()).unwrap_or('0').to_digit(10).unwrap_or(0) as u64;
    let last_number: u64 = line.chars().rev().find(|c| c.is_numeric()).unwrap_or('0').to_digit(10).unwrap_or(0) as u64;

    first_number * 10 + last_number
}
