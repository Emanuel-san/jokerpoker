mod card;
mod hand;
mod deck;

//use card::*;
use hand::*;
use deck::*;
use card::*;

//use crate::card::CardSuit;
//use crate::CardSuit::*;
//use rand::prelude::*;


fn print_hand(hand_obj: &Hand) {
    let Hand{
        hand_vec: temp_hand
    } = hand_obj;
    for card in temp_hand{
        if card.value != 14{
            print!("{} of ", card.value);
        }
        match card.suit {
            CardSuit::Joker => print!("Joker | "),
            CardSuit::Diamond => print!("Diamonds | "),
            CardSuit::Spade => print!("Spades | "),
            CardSuit::Clove => print!("Cloves | "),
            CardSuit::Heart => print!("Hearts | "),
        }
    }
}


fn main() {
    let mut deck_of_cards = Deck::get_deck();
    let mut five_card_hand = Hand::draw_five_card_hand(&mut deck_of_cards);

    //println!("{:?}", deck_of_cards);
    //println!("{:?}", five_card_hand);
    let Deck {
        deck_vec: deck_destructred
    } = &deck_of_cards;

    println!("{}", deck_destructred.len());

    print_hand(&five_card_hand);
    five_card_hand.discard_card_from_hand(0);
    println!("");
    print_hand(&five_card_hand);
    five_card_hand.draw_until_five_cards(&mut deck_of_cards);
    println!("");
    print_hand(&five_card_hand);

    let Deck {
        deck_vec: deck_destructred
    } = &deck_of_cards;
    println!("{}", deck_destructred.len());
    five_card_hand.draw_until_five_cards(&mut deck_of_cards);
    


    
    // generate_five_card_hand(&mut five_card_hand);
    // //five_card_hand.sort_hand_by_value();
    // //println!("Five card hand: {:?}", five_card_hand);

    // let card = five_card_hand.hand_vec.remove(1);
    // discarded_cards.push(card);
    // discarded_cards.push(five_card_hand.discard_card_from_hand(3));
    // //five_card_hand.discard_card_from_hand(3);
    // //println!("Five card hand: {:?}", five_card_hand.hand_vec);
    // //println!("Discarded: {:?}", discarded_cards);
    // generate_five_card_hand(&mut five_card_hand);
    // print_hand(&five_card_hand);
}