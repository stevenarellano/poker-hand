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
