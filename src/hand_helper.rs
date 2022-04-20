use crate::cards;

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

pub fn get_suit(hand: Vec<cards::Card>, suit: &str) -> Vec<cards::Card> {
    let mut filtered = hand.clone();

    return filtered
        .into_iter()
        .filter(|card| card.suit.as_str() == suit)
        .collect::<Vec<_>>();
}
