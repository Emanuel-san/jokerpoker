use clearscreen::ClearScreen; //just used to clear terminal, not the prettiest...
use std::io;

use crate::hand::*;
use crate::machine::*;
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
    pub fn chk_select_input(&self) -> Result<usize, ()> {
        if let Ok(input) = self.parse_input() {
            if input <= 5 && input >= 1 {
                Ok(input)
            } 
            else {
                Err(())
            }
        } 
        else {
            Err(())
        }
    }
    //Check if the parsed input is the allowed values of funds to be "inserted" into the "machine"
    fn chk_parsed_funds_input(&self) -> Result<usize, ()> {
        if let Ok(input) = self.parse_input() {
            if input == 1 || input == 2 || input == 5 || input == 10 {
                Ok(input)
            } 
            else {
                Err(())
            }
        } 
        else {
            Err(())
        }
    }

    ///Check if the parsed input is within the allowed span of 1-4 (Selecting card 1-4)
    fn chk_parsed_double_input(&self) -> Result<usize, ()> {
        if let Ok(input) = self.parse_input() {
            if input <= 4 && input >= 1 {
                Ok(input)
            } 
            else {
                Err(())
            }
        } 
        else {
            Err(())
        }
    }
    ///Only used during selection of cards to keep and discard
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
            *state = MachineState::CoinsAvailable;
        } else {
            if let Ok(parsed_input) = self.chk_select_input() { //check for valid input
                ClearScreen::default().clear().expect("failed to clear terminal");
                hand.hand_vec[parsed_input - 1].alter_selection(); //change if a card is selected or not (player selection)
                *holder = format_hand(&hand);
                print_hand_and_credits(&holder, &funds);
            } 
            else {
                println!("Invalid input");
            }
        }
    }

    pub fn double_input(&self) -> Option<usize> {

        if let Ok(parsed_input) = self.chk_parsed_double_input(){
            Some(parsed_input)
        } else {
            println!("Invalid input");
            None
        }
    }

    pub fn insert_funds(&self, funds: &mut Wallet, state: &mut MachineState) {
        if self.input_string.trim().to_lowercase() == "draw" {
            if funds.credits > 0 {
                *state = MachineState::CoinsAvailable;
            } else {
                println!("No available funds");
            }
        } else {
            if let Ok(input) = self.chk_parsed_funds_input() {
                ClearScreen::default().clear().expect("failed to clear terminal");
                funds.add_funds(&input);
                print_insert_coin();
            } else {
                println!("Invalid input");
            }
        }
    }

    pub fn win_input(&self, funds: &mut Wallet, state: &mut MachineState, credits_won: &usize, input_control: &mut bool) {
        if self.input_string.trim().to_lowercase() == "draw" {
            funds.add_funds(credits_won);
            *state = MachineState::CoinsAvailable;
            *input_control = true;
        } 
        else if self.input_string.trim().to_lowercase() == "withdraw" {
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

    pub fn end_input(&self, funds: &mut Wallet, state: &mut MachineState) -> bool{
        if self.input_string.trim().to_lowercase() == "draw"{
            ClearScreen::default().clear().expect("failed to clear terminal");
            true
        }
        else if self.input_string.trim().to_lowercase() == "withdraw"{
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
