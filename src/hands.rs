use crate::cards;
use crate::hand_helper::*;
use crate::hand_structs::*;

pub struct SuitCounter {
    spades: u16,
    hearts: u16,
    diamonds: u16,
    clubs: u16,
}

impl SuitCounter {
    fn any_flush(&self) -> bool {
        self.spades >= 5 || self.hearts >= 5 || self.diamonds >= 5 || self.clubs >= 5
    }
}

pub fn is_flush(hand: Vec<cards::Card>) -> bool {
    let mut suit_counter: SuitCounter = SuitCounter {
        spades: 0,
        hearts: 0,
        diamonds: 0,
        clubs: 0,
    };

    for card in hand.iter() {
        match card.suit.as_str() {
            "Spades" => suit_counter.spades += 1,
            "Hearts" => suit_counter.hearts += 1,
            "Diamonds" => suit_counter.diamonds += 1,
            "Clubs" => suit_counter.clubs += 1,
            _ => println!("NOT A SUIT"),
        }
    }

    suit_counter.any_flush()
}

// pub fn get_flush(hand: Vec<cards::Card>) -> Hands {}

// pub fn is_royal_flush(table: Vec<cards::Card>, hand: Vec<cards::Card>) -> bool {}

// pub fn calculate_hand(table: Vec<cards::Card>, hand: Vec<cards::Card>) -> Hands {}
