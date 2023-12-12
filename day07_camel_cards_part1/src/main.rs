use std::collections::HashMap;
#[allow(unused_imports)]
use day07_camel_cards_common::{HandBid, SAMPLE_DATA};

fn main() {
    let result = do_work(SAMPLE_DATA);
    println!("{}", result);
}

fn do_work<const N: usize>(data: [HandBid; N]) -> u64 {
    let processed_hands: [ProcessedHandBid; N] = process_hands(data);

    processed_hands[0].bid
}

#[derive(PartialEq, Eq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King, 
    Ace,
}

#[derive(PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct ProcessedHandBid {
    hand: [Card; 5],
    hand_type: HandType,
    bid: u64,
}

fn process_hands<const N: usize>(raw_hands: [HandBid; N]) -> [ProcessedHandBid; N] {
    let processed_hands: [ProcessedHandBid; N] = raw_hands.map(|hand_bid| {
        process_hand(hand_bid)
    });

    processed_hands
}

fn process_hand(raw_hand: HandBid) -> ProcessedHandBid {
    let hand: [Card; 5] = raw_hand.hand.map(|card| {
        map_card(card)
    });
    let hand_type: HandType = compute_hand_type(&hand);
    ProcessedHandBid {
        hand,
        hand_type,
        bid: raw_hand.bid,
    }
}

fn map_card(raw_card: char) -> Card {
    match raw_card {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Invalid card: {}", raw_card),
    }
}

fn compute_hand_type(hand: &[Card; 5]) -> HandType {
    let mut card_counts: HashMap<Card, u8> = HashMap::new();
    for card in hand {
        let count = card_counts.entry(*card).or_insert(0);
        *count += 1;
    }

    let mut highest_count: Option<u8> = None;
    let mut second_highest_count: Option<u8> = None;

    for (_card, count) in card_counts {
        if highest_count.is_none() || count > highest_count.unwrap() {
            second_highest_count = highest_count;
            highest_count = Some(count);
        } else if second_highest_count.is_none() || count > second_highest_count.unwrap() {
            second_highest_count = Some(count);
        }
    }

    match highest_count {
        Some(5) => HandType::FiveOfAKind,
        Some(4) => HandType::FourOfAKind,
        Some(3) => {
            match second_highest_count {
                Some(2) => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            }
        },
        Some(2) => {
            match second_highest_count {
                Some(2) => HandType::TwoPair,
                _ => HandType::OnePair,
            }
        },
        _ => HandType::HighCard,
    }
}