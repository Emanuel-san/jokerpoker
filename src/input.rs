use std::io;

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
}
