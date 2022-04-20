mod cards;
mod hands;

fn main() {
    let mut deck: Vec<cards::Card> = cards::create_deck();
    println!("{:?}", deck.len());
    println!("{:?}", cards::deal_flop(&mut deck));
    println!("{:?}", deck.len());

    let mut flop: Vec<cards::Card> = cards::deal_flop(&mut deck);
    let mut hand = cards::deal_hand(&mut deck);
    let mut flop2: Vec<cards::Card> = cards::deal_flop(&mut deck);

    flop.append(&mut hand);
    flop.append(&mut flop2);
    println!("{:?}", flop);
    flop = hands::sort_by_suit(flop);
    println!("{:?}", flop);
    flop = hands::sort_by_num(flop);
    println!("{:?}", flop);
    flop = hands::sort_by_both(flop);
    println!("{:?}", flop)
}
