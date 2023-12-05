#[allow(unused_imports)]
use day04_scratchcards_common::{ScratchCard, REAL_DATA, SAMPLE_DATA};
use std::collections::HashSet;

fn main() {
    let result = do_work(REAL_DATA);
    println!("{}", result);
}

fn do_work<const W: usize, const N: usize, const C: usize>(data: [ScratchCard<W, N>; C]) -> u64 {
    let mut card_counts: [u64; C] = [1; C];

    for i in 0..C {
        let copies = card_counts[i];
        let matches = get_matches(&data[i]);

        for j in 1..matches + 1 {
            if (i + j) < C {
                card_counts[i + j] += copies;
            }
        }
    }

    card_counts.iter().sum()
}

fn get_matches<const W: usize, const N: usize>(card: &ScratchCard<W, N>) -> usize {
    let winning_numbers = HashSet::from(card.winning_numbers);
    let numbers = HashSet::from(card.numbers);

    winning_numbers.intersection(&numbers).count()
}
