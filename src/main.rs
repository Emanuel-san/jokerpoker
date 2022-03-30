mod card;
mod hand;
//use card::*;
use hand::*;

//use crate::card::CardSuit;
//use crate::CardSuit::*;
//use rand::prelude::*;

fn generate_five_card_hand(empty_hand: &mut Hand){
    while empty_hand.hand_vec.len() < 5 { //loop untill vec holds 5 elements
        empty_hand.push_card_to_hand();
    }
}
fn main() {
    let mut five_card_hand = Hand::new();
    generate_five_card_hand(&mut five_card_hand);

    println!("{:?}", five_card_hand);
    // let Hand{ 
    //     hand_vec: deconstructed_hand
    //     } = five_card_hand;


    
}