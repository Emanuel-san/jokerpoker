use crate::hand::*;
use std::*;

#[derive(Debug)]
pub struct CharHolder {
    holder: Vec<char>,
}

pub fn print_insert_coin() {
    println!(
        "
██ ███    ██ ███████ ███████ ██████  ████████      ██████  ██████  ██ ███    ██ 
██ ████   ██ ██      ██      ██   ██    ██        ██      ██    ██ ██ ████   ██ 
██ ██ ██  ██ ███████ █████   ██████     ██        ██      ██    ██ ██ ██ ██  ██ 
██ ██  ██ ██      ██ ██      ██   ██    ██        ██      ██    ██ ██ ██  ██ ██ 
██ ██   ████ ███████ ███████ ██   ██    ██         ██████  ██████  ██ ██   ████
Accepted values of funds: 1, 2, 5 and 10.
"
    )
}

pub fn format_hand(hand: &Hand) -> Vec<CharHolder> {
    let mut vec_holder: Vec<CharHolder> = Vec::new();
    for card in &hand.hand_vec {
        let mut char_holder = CharHolder::new();
        let face = card.get_face_bytes();
        let suit = card.get_suit_bytes();
        if card.selected == true {
            let mut formated_card = format!(
                "
┌──────────────┐
|{}            |
|   SELECTED   |
|              |
|              |
|       {}      |
|              |
|              |
|   SELECTED   |
|            {}|
└──────────────┘",
                str::from_utf8(&face).unwrap(),
                str::from_utf8(&suit).unwrap(),
                str::from_utf8(&face).unwrap()
            );
            formated_card = formated_card.replace("\n", "");

            char_holder.format_string_to_chars(&formated_card);

            vec_holder.push(char_holder);
        } else {
            let mut formated_card = format!(
                "
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
└──────────────┘",
                str::from_utf8(&face).unwrap(),
                str::from_utf8(&suit).unwrap(),
                str::from_utf8(&face).unwrap()
            );
            formated_card = formated_card.replace("\n", "");

            char_holder.format_string_to_chars(&formated_card);

            vec_holder.push(char_holder);
        }
    }
    vec_holder
}

pub fn print_hand(vec_of_charholder: &Vec<CharHolder>) {
    for row in 0..11 {
        for card in vec_of_charholder {
            for char in (row * 16)..(row * 16) + 16 {
                print!("{}", card.holder[char]);
            }
        }
        println!("");
    }
}

impl CharHolder {
    fn new() -> Self {
        Self { holder: Vec::new() }
    }

    fn format_string_to_chars(&mut self, formated_card: &String) {
        for char in formated_card.chars() {
            self.holder.push(char)
        }
    }
}
