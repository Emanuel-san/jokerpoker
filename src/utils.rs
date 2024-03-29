use crate::hand::*;
use crate::logic::*;
use std::*;

///Character holder data type
#[derive(Debug)]
pub struct CharHolder {
    holder: Vec<char>,
}

///prints "Insert coin" ASCII-art in terminal
pub fn print_insert_coin() {
    println!("
██ ███    ██ ███████ ███████ ██████  ████████      ██████  ██████  ██ ███    ██ 
██ ████   ██ ██      ██      ██   ██    ██        ██      ██    ██ ██ ████   ██ 
██ ██ ██  ██ ███████ █████   ██████     ██        ██      ██    ██ ██ ██ ██  ██ 
██ ██  ██ ██      ██ ██      ██   ██    ██        ██      ██    ██ ██ ██  ██ ██ 
██ ██   ████ ███████ ███████ ██   ██    ██         ██████  ██████  ██ ██   ████
Accepted values of funds: 1, 2, 5 and 10.
    ")
}

///Formatted ASCII art for a face-down card
fn add_face_down(vec_holder: &mut Vec<CharHolder>) {
    let mut char_holder = CharHolder::new();
    let mut face_down_card = String::from("
┌──────────────┐
|**************|
|*█ █ █ █ █ █ *|
|* █ █ █ █ █ █*|
|*█ █ █ █ █ █ *|
|* █ █ █ █ █ █*|
|*█ █ █ █ █ █ *|
|* █ █ █ █ █ █*|
|*█ █ █ █ █ █ *|
|**************|
└──────────────┘
    ");
    face_down_card = face_down_card.replace("\n", "");
    char_holder.format_string_to_chars(&face_down_card);
    vec_holder.push(char_holder);
}

///Formats each card in the hand into a vector of characters which is then used for visuall printing of cards
pub fn format_hand(hand: &Hand) -> Vec<CharHolder> {
    let mut vec_holder: Vec<CharHolder> = Vec::new();
    for card in &hand.hand_vec {
        let mut char_holder = CharHolder::new();
        let face = card.get_face_bytes();//collect card value character as bytes
        let suit = card.get_suit_bytes();//collect suit character as bytes
        if card.selected == true {
            let mut formated_card = format!("
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
                str::from_utf8(&face).unwrap(),//convert the bytes to a string slice
                str::from_utf8(&suit).unwrap(),
                str::from_utf8(&face).unwrap()
            );
            formated_card = formated_card.replace("\n", ""); //remove all new lines

            char_holder.format_string_to_chars(&formated_card); //format each card to a vector of characters

            vec_holder.push(char_holder);// a vector of vector of charachters
        } else {
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
    //there's only one face-up card when doubling so we push 4 formatted face down cards to the vector
    while vec_holder.len() < 5 {
        add_face_down(&mut vec_holder);
    }
    vec_holder
}

///Prints a 5 card hand in a row in the terminal
pub fn print_hand_and_credits(vec_of_charholder: &Vec<CharHolder>, current_funds: &Wallet) {
    println!("CREDITS: {}", current_funds.credits);
    for row in 0..11 { // a card contains 11 rows of lines
        for card in vec_of_charholder { // iterate through each card...
            for char in (row * 16)..(row * 16) + 16 { //...and 16 characters for each row and card.
                print!("{}", card.holder[char]);
            }
        }
        println!("");
    }
}
///Matches each state with the correct tips string and prints it.
pub fn print_tips(state: &MachineState){
    match *state {
        MachineState::CreditsAvailable => println!(r#"Type "draw" to continue playing or "withdraw" to cash out."#),
        MachineState::CardSelection => println!(r#"Select and de-select cards by entering the card position(1-5) and "draw" to change unselected cards."#),
        MachineState::Win => println!(r#"Would you like to "double" your winnings, "draw" new hand or "withdraw" your credits?"#),
        MachineState::Double => println!(r#"Beat the dealers card by choosing one of the face down cards (1-4) that is higher."#),
        _ => ()
    }
}

impl CharHolder {
    fn new() -> Self {
        Self { holder: Vec::new() }
    }
    ///iterate through each charachter in the string and push it to the vector
    fn format_string_to_chars(&mut self, formated_card: &String) {
        for char in formated_card.chars() {
            self.holder.push(char)
        }
    }
}
