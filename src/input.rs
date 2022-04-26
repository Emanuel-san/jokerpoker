use std::{io, num::ParseIntError};

use crate::utils::CharHolder;

const RADIX: u32 = 10;
pub struct UserInput{
    input_string: String
}
pub struct UsableInput<T>{
    input: T
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

    fn retain_digits(&mut self){
        self.input_string.retain(|a| a.is_digit(RADIX));
    }

    pub fn parse_input_to_digits(&mut self) //-> Result<UsableInput<Vec<u8>>, ParseIntError>
    {
        self.retain_digits();
        let mut input_digits:Vec<u32> = Vec::new(); 
        for digit in self.input_string.chars().filter_map(|a| a.to_digit(RADIX)){
            input_digits.push(digit);
        }
        

        println!("{:?}", input_digits);
    }
}

impl<T> UsableInput<T> {

    pub fn contents(&self) -> &T{
        &self.input
    }
}

