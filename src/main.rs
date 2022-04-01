mod card;
mod hand;
mod deck;

use hand::*;
use deck::*;
use card::*;


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
    
}