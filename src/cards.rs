mod misc;

#[derive(Clone, Debug)]
pub struct Card {
    pub suit: String,
    pub number: u16,
}

pub fn create_deck() -> Vec<Card> {
    let suits: Vec<&str> = vec!["Hearts", "Diamonds", "Clubs", "Spades"];

    let mut deck = Vec::new();
    for i in 0..4 {
        for j in 2..15 {
            deck.push(Card {
                suit: String::from(suits[i]),
                number: j as u16,
            })
        }
    }

    return deck;
}

pub fn get_card(deck_ptr: &mut Vec<Card>) -> Card {
    let ind = misc::range_rand(deck_ptr.len());
    deck_ptr.remove(ind)
}

pub fn deal_hand(deck_ptr: &mut Vec<Card>) -> Vec<Card> {
    let mut hand = Vec::new();

    for i in 0..2 {
        hand.push(get_card(deck_ptr));
    }

    hand
}

pub fn burn_card(deck_ptr: &mut Vec<Card>) {
    let ind = misc::range_rand(deck_ptr.len());
    deck_ptr.remove(ind);
}

pub fn deal_flop(deck_ptr: &mut Vec<Card>) -> Vec<Card> {
    let mut hand = Vec::new();

    for i in 0..3 {
        hand.push(get_card(deck_ptr));
    }

    hand
}

fn main() {}
