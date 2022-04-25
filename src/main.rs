
mod card;
mod hand;
mod deck;
mod printing;
mod machine;

use crate::printing::*;
use machine::*;
use hand::*;
use deck::*;
use card::*;





fn main() {
    let mut deck_of_cards = Deck::get_deck();
    let mut five_card_hand = Hand::draw_five_card_hand(&mut deck_of_cards);
    let mut holder: Vec<CharHolder> = Vec::new();
    format_hand(&mut holder, &five_card_hand);
    CharHolder::print_hand(&holder);

}