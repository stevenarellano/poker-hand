mod cards;
mod hands;

fn main() {
    let mut deck: Vec<cards::Card> = cards::create_deck();
    println!("{:?}", deck.len());
    println!("{:?}", cards::deal_flop(&mut deck));
    println!("{:?}", deck.len());

    let mut flop: Vec<cards::Card> = cards::deal_flop(&mut deck);
    let mut hand = cards::deal_hand(&mut deck);

    flop.append(&mut hand);
    println!("{:?}", flop);
    println!("{:?}", hands::sort_by_suit(flop));
}
