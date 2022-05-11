use clearscreen::ClearScreen;
use std::io;

use crate::card::*;
use crate::hand::*;
use crate::machine::*;
use crate::utils::*;
//use crate::deck::*;

#[derive(PartialEq)]
pub struct UserInput {
    pub input_string: String,
}

#[derive(PartialEq)]
pub enum Control {
    Accepted,
    Rejected
}

impl UserInput {
    pub fn get_user_input() -> Self {
        let mut input_string = String::new();
        io::stdin()
            .read_line(&mut input_string)
            .expect("UserInput::, Failed to read line");

        UserInput {
            input_string,
        }
    }

    pub fn parse_input(&self) -> Result<usize, ()> {
        if let Ok(parsed_input) = self.input_string.trim().parse::<usize>() {
            Ok(parsed_input)
        } else {
            Err(())
        }
    }

    pub fn chk_select_input(&self) -> Result<usize, ()> {
        if let Ok(input) = self.parse_input() {
            if input <= 5 && input >= 1 {
                Ok(input)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn chk_draw_input(&self) -> Result<(), ()> {
        if self.input_string.trim().to_lowercase() == "draw" {
            Ok(())
        } else {
            Err(())
        }
    }

    fn chk_withdraw_input(&self) -> Result<(), ()> {
        if self.input_string.trim().to_lowercase() == "withdraw" {
            Ok(())
        } else {
            Err(())
        }
    }

    fn chk_parsed_funds_input(&self) -> Result<usize, ()> {
        if let Ok(input) = self.parse_input() {
            if input == 1 || input == 2 || input == 5 || input == 10 {
                Ok(input)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
    fn chk_parsed_double_input(&self) -> Result<usize, ()> {
        if let Ok(input) = self.parse_input() {
            if input <= 5 && input >= 2 {
                Ok(input)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn card_selection(
        &self,
        hand: &mut Hand,
        holder: &mut Vec<CharHolder>,
        state: &mut MachineState,
        funds: &Funds,
        ) {
        if let Ok(()) = self.chk_draw_input() {
            ClearScreen::default().clear().expect("failed to clear terminal");
            *state = MachineState::CoinsAvailable;
        } else {
            if let Ok(parsed_input) = self.chk_select_input() {
                ClearScreen::default().clear().expect("failed to clear terminal");
                let card: &mut Card = &mut hand.hand_vec[parsed_input - 1];
                card.alter_selection();
                *holder = format_hand(&hand);
                print_hand_and_credits(&holder, &funds);
            } else {
                println!("Invalid input");
            }
        }
    }
    pub fn double_input(&self, input_control: &mut Control) -> usize{

        if let Ok(parsed_input) = self.chk_parsed_double_input(){
            *input_control = Control::Accepted;
            parsed_input
        } else {
            println!("Invalid input");
            0
        }
    }

    pub fn funds_input(&self, funds: &mut Funds, state: &mut MachineState) {
        if let Ok(()) = self.chk_draw_input() {
            if funds.credits > 0 {
                ClearScreen::default().clear().expect("failed to clear terminal");
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

    pub fn win_input(&self, funds: &mut Funds, state: &mut MachineState, amount_won: &usize) {
        if let Ok(()) = self.chk_draw_input() {
            funds.add_funds(amount_won);
            *state = MachineState::CoinsAvailable;
        } 
        else if let Ok(()) = self.chk_withdraw_input() {
            funds.credits = 0;
            *state = MachineState::InsertCoin;
        } 
        else if self.input_string.trim().to_lowercase() == "double" && *state == MachineState::Win{
            *state = MachineState::Double;
        } 
        else {
            println!("Invalid input");
        }
    }
}
