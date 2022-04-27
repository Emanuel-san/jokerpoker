use std::io;

use crate::utils::*;
use crate::card::*;
use crate::hand::*;
use crate::CharHolder;
use crate::machine::*;
use clearscreen::ClearScreen;

#[derive(PartialEq)]
pub struct UserInput{
    pub input_string: String
}

impl UserInput {
    pub fn get_user_input() -> Self {

        let mut string_input = String::new();
        io::stdin()
            .read_line(&mut string_input)
            .expect("input::discard_hand_index, Failed to read line");
        
        Self {
            input_string: string_input
        }
    }

    pub fn parse_and_chk_select_input(&mut self) -> Result<usize, ()>
    {
       if let Ok(parsed_input) = self.input_string.trim().parse::<usize>(){
           if parsed_input <= 5 && parsed_input >= 1{
                return Ok(parsed_input)
           }
            println!("Invalid input");
            return Err(())
       } else {
            println!("Invalid input");
            return Err(())
       }
    }

    pub fn card_selection(&mut self, hand: &mut Hand, holder: &mut Vec<CharHolder>, state: &mut MachineState){
        if self.input_string.trim().to_lowercase() == "draw"{
            ClearScreen::default().clear().expect("failed to clear");
            *state = MachineState::FundsAvailable;
        } else {
            if let Ok(parsed_input) = self.parse_and_chk_select_input(){
                ClearScreen::default().clear().expect("failed to clear");
                let card: &mut Card = &mut hand.hand_vec[parsed_input - 1];
                card.alter_selection();
                *holder = format_hand(&hand);
                print_hand(&holder);
            }
        }
    }
}

