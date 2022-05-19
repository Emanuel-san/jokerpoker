use clearscreen::ClearScreen; //just used to clear terminal, not the prettiest...
use std::io;

use crate::hand::*;
use crate::logic::*;
use crate::utils::*;

#[derive(PartialEq)]
pub struct UserInput {
    pub input_string: String,
}

impl UserInput {
    //Just to simplify read stdin to one fn call
    pub fn get_user_input() -> Self {
        let mut input_string = String::new();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        UserInput {
            input_string,
        }
    }

    ///Try and parse the input
    pub fn parse_input(&self) -> Result<usize, ()> {
        if let Ok(parsed_input) = self.input_string.trim().parse::<usize>() {
            Ok(parsed_input)
        } 
        else {
            Err(())
        }
    }
    ///Check if the parsed input is within the allowed span of 1-5 (Selecting card 1-5)
    pub fn chk_select_input(&self) -> Option<usize> {
        if let Ok(input) = self.parse_input() {
            if input <= 5 && input >= 1 {
                Some(input)
            } 
            else {
                None
            }
        } 
        else {
            None
        }
    }
    //Check if the parsed input is the allowed values of funds to be "inserted" into the "machine"
    fn chk_parsed_funds_input(&self) -> Option<usize> {
        if let Ok(input) = self.parse_input() {
            if input == 1 || input == 2 || input == 5 || input == 10 {
                Some(input)
            } 
            else {
                None
            }
        } 
        else {
            None
        }
    }

    ///Check if the parsed input is within the allowed span of 1-4 (Selecting card 1-4)
    pub fn chk_parsed_double_input(&self) -> Option<usize> {
        if let Ok(input) = self.parse_input() {
            if input <= 4 && input >= 1 {
                Some(input)
            } 
            else {
                println!("Invalid input");
                None
            }
        } 
        else {
            println!("Invalid input");
            None
        }
    }
    ///Player inputs to select or de-select a card
    pub fn card_selection(
        &self,
        hand: &mut Hand,
        holder: &mut Vec<CharHolder>,
        state: &mut MachineState,
        funds: &Wallet,
        ) {
        if self.input_string.trim().to_lowercase() == "draw" { //Player inputs "draw" to get out of card selection state
            ClearScreen::default().clear().expect("failed to clear terminal");
            *state = MachineState::CreditsAvailable;
        } else {
            if let Some(parsed_input) = self.chk_select_input() { //check for valid input
                ClearScreen::default().clear().expect("failed to clear terminal");
                hand.alter_selected_card(parsed_input - 1); //change if a card in hand is selected or not (player selection)
                *holder = format_hand(&hand);
                print_hand_and_credits(&holder, &funds);
            } 
            else {
                println!("Invalid input");
            }
        }
    }
    ///Either adds funds or changes machine state depending on player input (Only used when state is InsertCoin)
    pub fn insert_funds(&self, funds: &mut Wallet, state: &mut MachineState) {
        if self.input_string.trim().to_lowercase() == "draw" {
            if funds.credits > 0 {
                *state = MachineState::CreditsAvailable;
            } else {
                println!("No available funds");
            }
        } else {
            if let Some(input) = self.chk_parsed_funds_input() { //check for valid input
                ClearScreen::default().clear().expect("failed to clear terminal");
                funds.add_funds(&input);
                print_insert_coin();
            } else {
                println!("Invalid input");
            }
        }
    }

    ///Available options after a win scenario and changes states accordingly to player input
    pub fn player_won_input(&self, funds: &mut Wallet, state: &mut MachineState, credits_won: &usize, input_control: &mut bool) {
        if self.input_string.trim().to_lowercase() == "draw" {
            funds.add_funds(credits_won); //add winnings to total available credits
            *state = MachineState::CreditsAvailable;
            *input_control = true; //this is to satisfy the while expression in main.rs line 83, else we prompt the player with choice a twice
        } 
        else if self.input_string.trim().to_lowercase() == "withdraw" {
            println!("Withdrawing {} credits", funds.credits);
            funds.credits = 0;
            *state = MachineState::InsertCoin;
        } 
        else if self.input_string.trim().to_lowercase() == "double"{
            *state = MachineState::Double;
        } 
        else {
            println!("Invalid input");
        }
    }
    //Available options after a loose scenario, boolean is returned depending on input validation
    pub fn player_lost_input(&self, funds: &mut Wallet, state: &mut MachineState) -> bool{
        if self.input_string.trim().to_lowercase() == "draw"{
            ClearScreen::default().clear().expect("failed to clear terminal");
            true
        }
        else if self.input_string.trim().to_lowercase() == "withdraw"{
            println!("Withdrawing {} credits", funds.credits);
            funds.credits = 0;
            *state = MachineState::InsertCoin;
            print_insert_coin();
            true
        }
        else{
            println!("Invalid input");
            false
        }
    }
}
