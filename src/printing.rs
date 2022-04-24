use std::*;

#[derive(Debug)]
pub struct CharHolder{
    holder: Vec<char>
}

impl CharHolder{    
    fn new() -> Self {
        let new_holder = Vec::new();
        
        Self {
            holder: new_holder
        }
    }

    pub fn format_card_to_string(face: Vec<u8>, suit: Vec<u8>) -> String{
        let mut formated_card = format!("
┌──────────────┐
|{}            |
|              |
|              |
|              |
|       {}      |
|              |
|              |
|              |
|            {}|
└──────────────┘", str::from_utf8(&face).unwrap(), str::from_utf8(&suit).unwrap(), str::from_utf8(&face).unwrap());
    formated_card = formated_card.replace("\n", "");
    formated_card
    }

    pub fn push_to_holder(&mut self, formated_card: &String){
        
        for char in formated_card.chars() {
            self.holder.push(char)
        }
    }

    pub fn print_hand(vec_of_charholder: &Vec<CharHolder>){
        for row in 0..11 {
            for card in 0..5{
                for char in (row * 16)..(row * 16) + 16{
                    print!("{}", vec_of_charholder[card].holder[char]);
                }
            }
            println!("");
        }
    }
}
