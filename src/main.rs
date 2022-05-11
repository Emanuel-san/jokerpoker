mod card;
mod deck;
mod hand;
mod input;
mod machine;
mod utils;

use crate::input::*;
use crate::utils::*;
use deck::*;
use hand::*;
use machine::*;

fn main() {
    let mut current_funds = Funds::new();
    let mut state = MachineState::InsertCoin;
    print_insert_coin();

    while state == MachineState::InsertCoin {
        println!("CREDITS: {}", current_funds.credits);
        let mut player_input = UserInput::get_user_input();
        player_input.funds_input(&mut current_funds, &mut state);

        while state == MachineState::CoinsAvailable {
            current_funds.reduce_funds();
            let mut deck_of_cards = Deck::get_deck();
            let mut five_card_hand = Hand::new();
            //five_card_hand.draw_card(&mut deck_of_cards);
            five_card_hand.draw_until_five_cards(&mut deck_of_cards);
            let mut holder = format_hand(&five_card_hand);
            print_hand_and_credits(&holder, &current_funds);
            println!("{:?}", five_card_hand);
            state = MachineState::CardSelection;

            while state == MachineState::CardSelection {
                player_input = UserInput::get_user_input();
                player_input.card_selection(
                    &mut five_card_hand,
                    &mut holder,
                    &mut state,
                    &current_funds,
                );
            }

            five_card_hand.discard_selected();
            five_card_hand.draw_until_five_cards(&mut deck_of_cards);
            holder = format_hand(&five_card_hand);
            print_hand_and_credits(&holder, &current_funds);
            let evaluation = evaluate_hand(&five_card_hand);
            evaluation.chk_evaluation_for_win(&mut state);
            println!("{:?}", five_card_hand);
            let mut credits_won = evaluation.hand_value;

            while state == MachineState::Win {
                println!(r#"Would you like to "double" your winnings, "draw" new hand or "withdraw" your credits?"#);
                player_input = UserInput::get_user_input();
                player_input.win_input(&mut current_funds, &mut state, &credits_won);

                while state == MachineState::Double {
                    let mut doubling_deck = Deck::get_deck();
                    let mut doubling_hand = Hand::new();
                    let mut input_control = Control::Rejected;
                    let mut selected_index: usize = 0;
                    doubling_hand.draw_card(&mut doubling_deck);
                    holder = format_hand(&doubling_hand);
                    print_hand_and_credits(&holder, &current_funds);
                    println!("  DEALERS CARD\n");
                    while input_control == Control::Rejected{
                        player_input = UserInput::get_user_input();
                        selected_index = player_input.double_input(&mut input_control) - 1;
                    }
                    doubling_hand.draw_until_five_cards(&mut doubling_deck);
                    doubling_hand.hand_vec[selected_index].alter_selection();
                    holder = format_hand(&doubling_hand);
                    print_hand_and_credits(&holder, &current_funds);
                    evaluate_doubling(&doubling_hand, &mut credits_won, &selected_index, &mut state);
                }
            }
            current_funds.chk_funds(&mut state);
        }
    }
}
