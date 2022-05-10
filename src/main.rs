mod card;
mod deck;
mod hand;
mod input;
mod machine;
mod utils;

use crate::input::*;
use crate::utils::*;
use clearscreen::ClearScreen;
use deck::*;
use hand::*;
use machine::*;

fn main() {
    ClearScreen::default()
        .clear()
        .expect("failed to clear terminal");
    let mut current_funds = Funds::new();
    let mut state = MachineState::InsertCoin;
    print_insert_coin();

    while state == MachineState::InsertCoin {
        println!("CREDITS: {}", current_funds.credits);
        let funds_input = UserInput::get_user_input();
        current_funds.chk_funds_input(&funds_input, &mut state);

        while state == MachineState::CoinsAvailable {
            current_funds.reduce_funds();
            let mut deck_of_cards = Deck::get_deck();
            let mut five_card_hand = Hand::new();
            five_card_hand.draw_until_five_cards(&mut deck_of_cards);
            let mut holder = format_hand(&five_card_hand);
            println!("CREDITS: {}", current_funds.credits);
            print_hand(&holder);
            state = MachineState::CardSelection;

            while state == MachineState::CardSelection {
                let select_input = UserInput::get_user_input();
                card_selection(
                    &select_input,
                    &mut five_card_hand,
                    &mut holder,
                    &mut state,
                    &current_funds,
                );
            }

            five_card_hand.discard_selected();
            five_card_hand.draw_until_five_cards(&mut deck_of_cards);
            holder = format_hand(&five_card_hand);
            print_hand(&holder);
            let evaluation = evaluate_hand(&five_card_hand);

            evaluation.print_evaluation();
            
            while state == MachineState::Double {}

            current_funds.chk_funds(&mut state);
        }
    }
}
