#[allow(unused_imports)]
use day07_camel_cards_common::{HandBid, REAL_DATA, SAMPLE_DATA};
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let result = do_work(REAL_DATA);
    println!("{}", result);
}

fn do_work<const N: usize>(data: [HandBid; N]) -> u64 {
    let mut processed_hands: Vec<ProcessedHandBid> = process_hands(data);
    processed_hands.sort();

    let mut total_winnings: u64 = 0;

    for (i, hand) in processed_hands.iter().enumerate() {
        let rank: u64 = (i + 1) as u64;

        let hand_winnings: u64 = rank * hand.bid;
        total_winnings += hand_winnings;
    }

    total_winnings
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq)]
struct ProcessedHandBid {
    hand: [Card; 5],
    hand_type: HandType,
    bid: u64,
}

impl Ord for ProcessedHandBid {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.hand_type.cmp(&other.hand_type);
        if ordering != Ordering::Equal {
            return ordering;
        }

        for i in 0..self.hand.len() {
            let ordering = self.hand[i].cmp(&other.hand[i]);
            if ordering != Ordering::Equal {
                return ordering;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for ProcessedHandBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ProcessedHandBid {
    fn eq(&self, other: &Self) -> bool {
        let mut equal: bool = self.hand_type == other.hand_type;

        if equal {
            for i in 0..self.hand.len() {
                equal = equal && self.hand[i] == other.hand[i];
            }
        }

        equal
    }
}

fn process_hands<const N: usize>(raw_hands: [HandBid; N]) -> Vec<ProcessedHandBid> {
    let mut processed_hands: Vec<ProcessedHandBid> = Vec::with_capacity(N);

    for raw_hand in raw_hands {
        let processed_hand: ProcessedHandBid = process_hand(raw_hand);
        processed_hands.push(processed_hand);
    }

    processed_hands
}

fn process_hand(raw_hand: HandBid) -> ProcessedHandBid {
    let hand: [Card; 5] = raw_hand.hand.map(map_card);
    let hand_type: HandType = compute_hand_type(&hand);
    ProcessedHandBid {
        hand,
        hand_type,
        bid: raw_hand.bid,
    }
}

fn map_card(raw_card: char) -> Card {
    match raw_card {
        'J' => Card::Joker,
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Invalid card: {}", raw_card),
    }
}

fn compute_hand_type(hand: &[Card; 5]) -> HandType {
    let mut card_counts: HashMap<Card, u8> = HashMap::new();
    let mut joker_count: u8 = 0;
    for card in hand {
        if card == &Card::Joker {
            // A Joker can be used as any card, so keep track separately
            joker_count += 1;
        } else {
            let count = card_counts.entry(*card).or_insert(0);
            *count += 1;
        }
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

    // Add the jokers to the highest count
    highest_count = match highest_count {
        Some(count) => Some(count + joker_count),
        None => Some(joker_count),
    };

    match highest_count {
        Some(5) => HandType::FiveOfAKind,
        Some(4) => HandType::FourOfAKind,
        Some(3) => match second_highest_count {
            Some(2) => HandType::FullHouse,
            _ => HandType::ThreeOfAKind,
        },
        Some(2) => match second_highest_count {
            Some(2) => HandType::TwoPair,
            _ => HandType::OnePair,
        },
        _ => HandType::HighCard,
    }
}
