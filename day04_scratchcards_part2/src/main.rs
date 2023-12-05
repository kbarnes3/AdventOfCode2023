#[allow(unused_imports)]
use day04_scratchcards_common::{ScratchCard, REAL_DATA, SAMPLE_DATA};
use std::collections::HashSet;

fn main() {
    let result = do_work(SAMPLE_DATA);
    println!("{}", result);
}

fn do_work<const W: usize, const N: usize, const C: usize>(data: [ScratchCard<W, N>; C]) -> u64 {
    data.iter().map(get_point_value).sum()
}

fn get_point_value<const W: usize, const N: usize>(card: &ScratchCard<W, N>) -> u64 {
    let winning_numbers = HashSet::from(card.winning_numbers);
    let numbers = HashSet::from(card.numbers);

    let matching_numbers = winning_numbers.intersection(&numbers).count();

    if matching_numbers == 0 {
        0
    } else {
        2u64.pow((matching_numbers as u32) - 1)
    }
}
