use std::io;

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
}

