
mod card;
mod hand;
mod deck;
mod utils;
mod machine;
mod input;

use crate::input::*;
use crate::utils::*;
use clearscreen::ClearScreen;
use machine::*;
use hand::*;
use deck::*;
use card::*;





fn main() {
    // let mut deck_of_cards = Deck::get_deck();
    // let mut five_card_hand = Hand::new();
    // let mut holder: Vec<CharHolder> = Vec::new();
    // five_card_hand.draw_until_five_cards(&mut deck_of_cards);
    // holder = format_hand(&five_card_hand);
    
    // print_hand(&holder);

    // five_card_hand.discard_card_from_hand(0);
    // five_card_hand.draw_until_five_cards(&mut deck_of_cards);
    // holder = format_hand(&five_card_hand);
    // print_hand(&holder);

    // println!("{}", evaluate_hand(&five_card_hand));
    let mut input = UserInput::get_user_input().parse_input_to_digits();


}