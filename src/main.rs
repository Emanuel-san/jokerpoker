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
    let mut current_funds = Wallet::new();
    let mut state = MachineState::InsertCoin;
    print_insert_coin();

    while state == MachineState::InsertCoin { //intial loop, no credits available
        println!("CREDITS: {}", current_funds.credits);
        let mut player_input = UserInput::get_user_input();
        player_input.insert_funds(&mut current_funds, &mut state);

        while state == MachineState::CreditsAvailable { // player input "draw"
            ClearScreen::default().clear().expect("failed to clear terminal");
            current_funds.reduce_funds(); //reduce credits by 1
            let mut deck_of_cards = Deck::get_deck(); //make a new deck for each hand
            let mut five_card_hand = Hand::new();
            //let mut debug_hand = Hand::new();
            //debug_hand._test_hand();
            five_card_hand.draw_until_five_cards(&mut deck_of_cards);
            let mut holder = format_hand(&five_card_hand);
            print_hand_and_credits(&holder, &current_funds);
            state = MachineState::CardSelection;

            while state == MachineState::CardSelection { //exits card selection when player inputs "draw"
                print_tips(&state);
                player_input = UserInput::get_user_input();
                player_input.card_selection(
                    &mut five_card_hand,
                    &mut holder,
                    &mut state,
                    &current_funds,);
            }

            five_card_hand.discard_selected(); // discard any card not selected...
            five_card_hand.draw_until_five_cards(&mut deck_of_cards); //... and fill up the hand with new cards
            holder = format_hand(&five_card_hand);
            print_hand_and_credits(&holder, &current_funds);
            let evaluation = Evaluation::evaluate_hand(&five_card_hand);
            evaluation.chk_evaluation_for_win(&mut state);
            let mut credits_won = evaluation.hand_value;
            let mut input_control = false;

            while state == MachineState::Win { //player had a winning hand
                print_tips(&state);
                player_input = UserInput::get_user_input();
                player_input.player_won_input(&mut current_funds, &mut state, &credits_won, &mut input_control); //player can either "draw", "withdraw" or "double"

                if state == MachineState::Double { // player chose to double the winnings
                    let mut doubling_deck = Deck::get_deck();// creates a new deck for doubling
                    let mut selected_index = None;
                    five_card_hand = Hand::new();
                    five_card_hand.draw_card(&mut doubling_deck); //add one face up card to the hand...
                    holder = format_hand(&five_card_hand); //... and 4 face down when formatting the cards
                    print_hand_and_credits(&holder, &current_funds);
                    println!("  DEALERS CARD\n"); // indicates which card is the dealers card
                    while selected_index == None { // exits when player input is valid
                        print_tips(&state);
                        player_input = UserInput::get_user_input();
                        selected_index = player_input.chk_parsed_double_input();
                    }
                    five_card_hand.draw_until_five_cards(&mut doubling_deck); // draw the rest of the cards
                    five_card_hand.alter_selected_card(selected_index.unwrap()); // set the card the player chose to selected 
                    holder = format_hand(&five_card_hand);
                    print_hand_and_credits(&holder, &current_funds);
                    println!("  DEALERS CARD\n");
                    evaluate_doubling(&five_card_hand, &mut credits_won, &selected_index.unwrap(), &mut state);
                }
            }
            
            current_funds.chk_funds(&mut state); // we set machine state to InsertCoin here if there are no credits left
            while input_control == false && state != MachineState::InsertCoin{ // enters when player had no winning hand or lost when doubling AND still has credits left, exits on correct input
                print_tips(&state);
                player_input = UserInput::get_user_input();
                input_control = player_input.player_lost_input(&mut current_funds, &mut state);
            }
        }
    }
}
