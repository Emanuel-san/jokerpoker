
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

    let state = MachineState::Playing;

    while state == MachineState::Playing {
        let mut deck_of_cards = Deck::get_deck();
        let mut five_card_hand = Hand::new();
        five_card_hand.draw_until_five_cards(&mut deck_of_cards);
        let mut holder = format_hand(&five_card_hand);
        print_hand(&holder);
        
        loop {
            let mut input = UserInput::get_user_input();
            if input.input_string.trim().to_lowercase() == "draw"{
                break;
            } else {
                if let Ok(parsed_input) = input.parse_and_chk_select_input(){
                    let mut card: &mut Card = &mut five_card_hand.hand_vec[parsed_input - 1];
                    if card.selected == true {
                        card.selected = false;
                    } else {
                        card.selected = true;
                    }
                }
            }
            holder = format_hand(&five_card_hand);
            print_hand(&holder);
        }
    }
}