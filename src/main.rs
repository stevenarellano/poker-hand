mod cards;
mod hand_helper;
mod hand_structs;
mod hands;
mod misc;

fn main() {
    let mut deck: Vec<cards::Card> = cards::create_deck();
    println!("{:?}", deck.len());
    println!("{:?}", cards::deal_flop(&mut deck));
    println!("{:?}", deck.len());

    let mut flop: Vec<cards::Card> = cards::deal_flop(&mut deck);
    let mut hand = cards::deal_hand(&mut deck);
    let mut flop2: Vec<cards::Card> = cards::deal_flop(&mut deck);

    let mut flush_example = vec![
        cards::Card {
            suit: String::from("Clubs"),
            number: 3,
        },
        cards::Card {
            suit: String::from("Clubs"),
            number: 7,
        },
        cards::Card {
            suit: String::from("Clubs"),
            number: 4,
        },
        cards::Card {
            suit: String::from("Clubs"),
            number: 9,
        },
        cards::Card {
            suit: String::from("Diamonds"),
            number: 12,
        },
        cards::Card {
            suit: String::from("Clubs"),
            number: 7,
        },
        cards::Card {
            suit: String::from("Spades"),
            number: 14,
        },
    ];

    let mut rando = vec![
        cards::Card {
            suit: String::from("Diamonds"),
            number: 3,
        },
        cards::Card {
            suit: String::from("Clubs"),
            number: 7,
        },
        cards::Card {
            suit: String::from("Clubs"),
            number: 4,
        },
        cards::Card {
            suit: String::from("Clubs"),
            number: 9,
        },
        cards::Card {
            suit: String::from("Diamonds"),
            number: 12,
        },
        cards::Card {
            suit: String::from("Clubs"),
            number: 7,
        },
        cards::Card {
            suit: String::from("Spades"),
            number: 14,
        },
    ];

    println!("{:?}", hand_helper::get_suit(flush_example, "Clubs"));
    println!("{:?}", hands::is_flush(rando));
}
