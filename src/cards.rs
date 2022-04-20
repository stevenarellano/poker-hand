mod misc;

#[derive(Debug)]
pub struct Card {
    suit: String,
    number: u16,
}

pub fn create_deck() -> Vec<Card> {
    let suits: Vec<&str> = vec!["Hearts", "Diamonds", "Clubs", "Spades"];

    let mut deck = Vec::new();
    for i in 0..4 {
        for j in 2..14 {
            deck.push(Card {
                suit: String::from(suits[i]),
                number: j as u16,
            })
        }
    }

    return deck;
}

pub fn get_card(deck: &mut Vec<Card>) -> Card {
    let ind = misc::range_rand(deck.len());
    deck.remove(ind)
}

pub fn deal_hand(deck: &mut Vec<Card>) -> Vec<Card> {
    let mut hand = Vec::new();

    for i in 0..2 {
        hand.push(get_card(deck));
    }

    hand
}

fn main() {}
