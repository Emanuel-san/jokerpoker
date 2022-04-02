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
            if card.value > 0{
                print!("{} of ", card.value + 1);
            } else {
                print! ("Ace of ");
            }
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

fn evaluate_hand(poker_hand: &Hand){
    let mut suit_counter = vec![0u8; 4];
    let mut value_counter = vec![0u8; 15];
    let mut jokers: u8 = 0;

    for card in &poker_hand.hand_vec{
        value_counter[card.value as usize] += 1;
        match card.suit {
            CardSuit::Diamond => suit_counter[0] += 1,
            CardSuit::Clove => suit_counter[1] += 1,
            CardSuit::Spade => suit_counter[2] += 1,
            CardSuit::Heart => suit_counter[3] += 1,
            CardSuit::Joker => jokers +=1
        }
    }
    // println!("Values: {:?}", value_counter);
    // println!("Suits: {:?}", suit_counter);
    // println!("Jokers: {}", jokers);
}


fn main() {
    let mut deck_of_cards = Deck::get_deck();
    let mut five_card_hand = Hand::draw_five_card_hand(&mut deck_of_cards);

    println!("{:?}", deck_of_cards);
    //println!("{:?}", five_card_hand);
    // let Deck {
    //     deck_vec: deck_destructred
    // } = &deck_of_cards;

    //println!("{}", deck_destructred.len());

    print_hand(&five_card_hand);
    five_card_hand.discard_card_from_hand(0);
    println!("");
    print_hand(&five_card_hand);
    five_card_hand.draw_until_five_cards(&mut deck_of_cards);
    println!("");
    print_hand(&five_card_hand);

    // let Deck {
    //     deck_vec: deck_destructred
    // } = &deck_of_cards;
    // println!("{}", deck_destructred.len());
    five_card_hand.draw_until_five_cards(&mut deck_of_cards);

    evaluate_hand(&five_card_hand);
    
}