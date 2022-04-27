use std::io;

pub struct UserInput{
    input_string: String
}

#[derive(Debug)]
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

    pub fn parse_and_chk_select_input(&mut self) -> Result<UsableInput<u8>, ()>
    {
       if let Ok(parsed_input) = self.input_string.trim().parse::<u8>(){
           if parsed_input <= 5 && parsed_input >= 1{
                return Ok(UsableInput{input: parsed_input})
           }
            println!("Invalid input");
            return Err(())
       } else {
            println!("Invalid input");
            return Err(())
       }
    }
}

impl<T> UsableInput<T> {

    pub fn contents(&self) -> &T{
        &self.input
    }
}

