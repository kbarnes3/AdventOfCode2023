#[allow(unused_imports)]
use day07_camel_cards_common::{HandBid, SAMPLE_DATA};

fn main() {
    let result = do_work(SAMPLE_DATA);
    println!("{}", result);
}

fn do_work<const N: usize>(data: [HandBid; N]) -> u64 {
    data[0].bid
}
