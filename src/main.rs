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
    println!("\n");
}

fn evaluate_hand(poker_hand: &Hand){
    let mut suit_tracker = vec![0u8; 4];
    let mut value_tracker = vec![0u8; 15];
    let mut jokers: u8 = 0;

    for card in &poker_hand.hand_vec{
        value_tracker[card.value as usize] += 1;
        match card.suit {
            CardSuit::Diamond => suit_tracker[0] += 1,
            CardSuit::Clove => suit_tracker[1] += 1,
            CardSuit::Spade => suit_tracker[2] += 1,
            CardSuit::Heart => suit_tracker[3] += 1,
            CardSuit::Joker => jokers +=1
        }
    }

    value_tracker[13] = value_tracker[0]; //We need to add aces to the highest value count aswell since we only count them as lowest when checking values in previous for loop.
    //println!("Values: {:?}", value_counter);
    // println!("Suits: {:?}", suit_counter);
    // println!("Jokers: {}", jokers);

    //filter out the suit_tracker where 0 values occur and collect the none zero values into a new vec (counted_suits)...
    let mut counted_suits = suit_tracker.into_iter()
    .filter(|&x| x > 0).collect::<Vec<_>>();
    counted_suits.sort_unstable();//... and sort it.
    counted_suits[0] += jokers; // add jokers to the index 0 incase they create a flush. (i.e index == 3 and we have 2 jokers, then we have a flush.)
    //println!("{:?}", counted_suits);
    let is_flush = counted_suits[0] == 5; // If index 0 contains element 5 then we have a flush.
    let mut is_straight = false;

    let mut vec_pointer = 14;
    //We use a "pointer" to avoid looking at cards again that was a part of another sequence.
    //This is only for checking if we have a poker hand with a straight.

    while vec_pointer > 3{// If the pointer reachs value 3 its no point at looking at the rest since Ace-2-3-4 will not make a straight. 
        let mut jokers_left = jokers; //Each time a sequence "fails" we will shadow this variable and re-declare jokers to use in the next sequence
        let mut straight_cards = 0; // Same as above though we reset to 0 on "failed" sequence.
        for i in (0..vec_pointer).rev(){ // represents the sequence (0..ptr) = 0-13
            println!("i: {} | ptr: {} | jokers: {} | straight_cards: {}| faces[i]: {}",i, vec_pointer, jokers_left, straight_cards, value_tracker[i]);
            if value_tracker[i] == 0 { // if element at index i is 0...
                if jokers_left == 0 { // ...then we check if we have jokers left to use.
                    break; // If not then we break out.
                }
                jokers_left -= 1; //Remove joker is used
            }
            else if i==vec_pointer-1 { //Since we use jokers we will only reduce vec_pointer if this is true, else its possible we miss a possible straight in the next sequence
                vec_pointer -= 1;
            }
            straight_cards += 1; //if we found a index with a none zero value or we used a joker, add 1 straight card.
        }
        vec_pointer -= 1; //always reduce by atleast one since even tho we used jokers we can guarantee the first index we check will not form a straight.
        if straight_cards == 5 { // We have a straight!
            is_straight = true;
            break;
        }
    }

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