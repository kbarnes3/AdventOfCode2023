#[allow(unused_imports)]
use day09_mirage_maintenance_common::{REAL_DATA, SAMPLE_DATA};

fn main() {
    let result = do_work(REAL_DATA);
    println!("{}", result);
}

fn do_work<const N: usize>(data: [&[i64]; N]) -> i64 {
    let mut result = 0;

    for line in data {
        result += predict_next_value(line);
    }

    result
}

fn predict_next_value(line: &[i64]) -> i64 {
    let mut predictions: Vec<Vec<i64>> = Vec::new();
    predictions.push(line.to_vec());

    while !contains_all_zeros(&predictions[predictions.len() - 1]) {
        let last_line = &predictions[predictions.len() - 1];
        let next_line = build_predictions(last_line);
        predictions.push(next_line);
    }

    let mut last_prediction = 0;

    for line in predictions.iter_mut().rev() {
        let last_value = line[line.len() - 1];
        last_prediction += last_value;
        line.push(last_prediction);
    }

    last_prediction
}

fn build_predictions(line: &Vec<i64>) -> Vec<i64> {
    let mut result = Vec::new();

    if line.len() <= 1 {
        panic!("Not enough values in line to build predictions")
    }

    for i in 0..line.len() - 1 {
        let prediction = line[i + 1] - line[i];

        result.push(prediction);
    }

    result
}

fn contains_all_zeros(line: &[i64]) -> bool {
    for value in line {
        if *value != 0 {
            return false;
        }
    }

    true
}
