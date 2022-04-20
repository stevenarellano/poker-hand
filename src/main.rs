mod cards;

fn main() {
    let mut deck: Vec<cards::Card> = cards::create_deck();
    println!("{:?}", deck.len());
    println!("{:?}", cards::deal_hand(&mut deck));
    println!("{:?}", deck.len());
}
