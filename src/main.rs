
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
    ClearScreen::default().clear().expect("failed to clear terminal");
    let mut current_funds = Funds::new();
    let mut state = MachineState::InsertCoin;
    print_insert_coin();
    
    while state == MachineState::InsertCoin{
        println!("Funds: {}", current_funds.coins);
        let funds_input = UserInput::get_user_input();
        Funds::add_funds(&funds_input, &mut current_funds, &mut state);

        while state == MachineState::FundsAvailable {
            current_funds.reduce_funds();
            let mut deck_of_cards = Deck::get_deck();
            let mut five_card_hand = Hand::new();
            five_card_hand.draw_until_five_cards(&mut deck_of_cards);
            let mut holder = format_hand(&five_card_hand);
            println!("Funds: {}", current_funds.coins);
            print_hand(&holder);
            state = MachineState::CardSelection;
            
            while state == MachineState::CardSelection {
                let select_input = UserInput::get_user_input();
                card_selection(&select_input, &mut five_card_hand, &mut holder, &mut state, &current_funds);
            }

            five_card_hand.discard_selected();
            five_card_hand.draw_until_five_cards(&mut deck_of_cards);
            holder = format_hand(&five_card_hand);
            print_hand(&holder);
            println!("{}", evaluate_hand(&five_card_hand));
            current_funds.chk_funds(&mut state);
        }
    }
}