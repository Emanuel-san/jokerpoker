
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





fn main() {
    ClearScreen::default().clear().expect("failed to clear");
    let mut coins = 0;
    let mut state = MachineState::FundsAvailable;
    print!("Insert coins to play: ");
    while state == MachineState::FundsAvailable {
        let mut deck_of_cards = Deck::get_deck();
        let mut five_card_hand = Hand::new();
        five_card_hand.draw_until_five_cards(&mut deck_of_cards);
        let mut holder = format_hand(&five_card_hand);
        print_hand(&holder);
        state = MachineState::CardSelection;
        
        while state == MachineState::CardSelection {
            let mut input = UserInput::get_user_input();
            input.card_selection(&mut five_card_hand, &mut holder, &mut state);
        }
        five_card_hand.discard_selected();
        five_card_hand.draw_until_five_cards(&mut deck_of_cards);
        holder = format_hand(&five_card_hand);
        print_hand(&holder);
        println!("{}", evaluate_hand(&five_card_hand));
    }
}