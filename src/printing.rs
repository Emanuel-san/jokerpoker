use std::*;

use crate::hand::*;

#[derive(Debug)]
pub struct CharHolder{
    holder: Vec<char>
}

pub fn format_hand(vec_holder: &mut Vec<CharHolder>, hand: &Hand){
    //let iter = &hand.hand_vec.into_iter().enumerate();
for card in &hand.hand_vec {
    let mut char_holder = CharHolder::new();
    let face = card.get_face_bytes();
    let suit = card.get_suit_bytes();
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

    char_holder.format_string_to_chars(&formated_card);
    
    vec_holder.push(char_holder);
    }
}

impl CharHolder{    
    fn new() -> Self {
        let new_holder = Vec::new();
        
        Self {
            holder: new_holder
        }
    }

    pub fn format_string_to_chars(&mut self, formated_card: &String){

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
