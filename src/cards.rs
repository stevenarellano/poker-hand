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

fn main() {}
