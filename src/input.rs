use clearscreen::ClearScreen;
use std::io;

use crate::card::*;
use crate::hand::*;
use crate::machine::*;
use crate::utils::*;

#[derive(PartialEq)]
pub struct UserInput {
    pub input_string: String,
}

impl UserInput {
    pub fn get_user_input() -> Self {
        let mut string_input = String::new();
        io::stdin()
            .read_line(&mut string_input)
            .expect("UserInput::, Failed to read line");

        Self {
            input_string: string_input,
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

    pub fn chk_draw_input(&self) -> Result<(), ()> {
        if self.input_string.trim().to_lowercase() == "draw" {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn chk_double_input(&self) -> Result<(), ()> {
        if self.input_string.trim().to_lowercase() == "double" {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn chk_withdraw_input(&self) -> Result<(), ()> {
        if self.input_string.trim().to_lowercase() == "withdraw" {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn chk_parsed_funds_input(&self) -> Result<usize, ()> {
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

    pub fn card_selection(
        &self,
        hand: &mut Hand,
        holder: &mut Vec<CharHolder>,
        state: &mut MachineState,
        funds: &Funds,
    ) {
        if let Ok(()) = self.chk_draw_input() {
            ClearScreen::default()
                .clear()
                .expect("failed to clear terminal");
            *state = MachineState::CoinsAvailable;
        } else {
            if let Ok(parsed_input) = self.chk_select_input() {
                ClearScreen::default()
                    .clear()
                    .expect("failed to clear terminal");
                let card: &mut Card = &mut hand.hand_vec[parsed_input - 1];
                card.alter_selection();
                *holder = format_hand(&hand);
                println!("Funds: {}", funds.credits);
                print_hand(&holder);
            } else {
                println!("Invalid input");
            }
        }
    }
}
