mod card;
mod hand;
//use card::*;
use hand::*;

use crate::card::{Card, CardSuit};

//use crate::card::CardSuit;
//use crate::CardSuit::*;
//use rand::prelude::*;

fn generate_five_card_hand(hand_obj: &mut Hand){
    while hand_obj.hand_vec.len() < 5 { //loop untill vec holds 5 elements
        hand_obj.draw_card_to_hand();
    }
}

fn print_hand(hand_obj: &Hand) {
    let Hand{
        hand_vec: temp_hand
    } = hand_obj;
    for card in temp_hand{
        print!("{} of ", card.value);
        match card.suit {
            CardSuit::Diamond => print!("Diamonds | "),
            CardSuit::Spade => print!("Spades | "),
            CardSuit::Clove => print!("Cloves | "),
            CardSuit::Heart => print!("Hearts | "),
        }
    }
}


fn main() {
    let mut five_card_hand = Hand::new();
    let mut discarded_cards: Vec<Card> = Vec::new();
    generate_five_card_hand(&mut five_card_hand);
    //five_card_hand.sort_hand_by_value();
    //println!("Five card hand: {:?}", five_card_hand);

    let card = five_card_hand.hand_vec.remove(1);
    discarded_cards.push(card);
    discarded_cards.push(five_card_hand.discard_card_from_hand(3));
    //five_card_hand.discard_card_from_hand(3);
    //println!("Five card hand: {:?}", five_card_hand.hand_vec);
    //println!("Discarded: {:?}", discarded_cards);
    generate_five_card_hand(&mut five_card_hand);
    print_hand(&five_card_hand);
}