use crate::cards;

pub struct RoyalFlush;

pub struct StraightFlush {
    pub suit: String,
    pub high_card: u16,
}

pub struct FourOfAKind {
    pub card: u16,
    pub kicker: u16,
}

pub struct FullHouse {
    pub set: Set,
    pub pair: OnePair,
}

pub struct Flush {
    pub card1: u16,
    pub card2: u16,
    pub card3: u16,
    pub card4: u16,
    pub card5: u16,
}

pub struct Straight {
    pub high_card: u16,
}

pub struct Set {
    pub set_number: u16,
    pub card4: u16,
    pub card5: u16,
}

pub struct TwoPair {
    pub high_pair: OnePair,
    pub low_pair: OnePair,
    pub kicker: u16,
}

pub struct OnePair {
    pub pair_number: u16,
    pub card3: u16,
    pub card4: u16,
    pub card5: u16,
}

pub struct HighCard {
    pub high_card: u16,
}

pub enum Hands {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    Set,
    TwoPair,
    OnePair,
    HighCard,
}

pub fn sort_by_suit(hand: Vec<cards::Card>) -> Vec<cards::Card> {
    let mut sorted = hand.clone();
    sorted.sort_by(|a, b| {
        a.suit
            .chars()
            .next()
            .unwrap()
            .partial_cmp(&b.suit.chars().next().unwrap())
            .unwrap()
    });

    sorted
}

pub fn sort_by_num(hand: Vec<cards::Card>) -> Vec<cards::Card> {
    let mut sorted = hand.clone();
    sorted.sort_by(|a, b| b.number.partial_cmp(&a.number).unwrap());

    sorted
}

pub fn sort_both_helper(a: &cards::Card, b: &cards::Card) -> std::cmp::Ordering {
    if b.suit.chars().next().unwrap() == a.suit.chars().next().unwrap() {
        b.number.partial_cmp(&a.number).unwrap()
    } else {
        a.suit
            .chars()
            .next()
            .unwrap()
            .partial_cmp(&b.suit.chars().next().unwrap())
            .unwrap()
    }
}

pub fn sort_by_both(hand: Vec<cards::Card>) -> Vec<cards::Card> {
    let mut sorted = hand.clone();
    sorted.sort_by(|a, b| sort_both_helper(a, b));

    sorted
}

// pub fn is_royal_flush(table: Vec<cards::Card>, hand: Vec<cards::Card>) -> bool {}

// pub fn calculate_hand(table: Vec<cards::Card>, hand: Vec<cards::Card>) -> Hands {}
