
mod card;
mod hand;
mod deck;


use hand::*;
use deck::*;
use card::*;

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
}

fn print_hand(hand_obj: &Hand) {
    let mut face: u8 = 0;
    let spade = '♠';
    let mut card_hand: Vec<String> = Vec::new();
    let mut char_holder:Vec<CharHolder> = Vec::new();
    for card in &hand_obj.hand_vec{
        match card.value {
            13 | 14 => (),
            12 => face = 75,
            11 => face = 81,
            10 => face = 74,
            0 => face = 65,
            number => face = number + 1,
        };
        // match card.suit {
        //     CardSuit::Joker => print!("Joker | "),
        //     CardSuit::Diamond => print!("Diamonds | "),
        //     CardSuit::Spade => print!("Spades | "),
        //     CardSuit::Clove => print!("Cloves | "),
        //     CardSuit::Heart => print!("Hearts | "),
        // }
        if face > 10{
            let drawn_card = format!("
┌──────────────┐
|{}             |
|              |
|              |
|              |
|       {}      |
|              |
|              |
|              |
|             {}|
└──────────────┘", face as char, spade, face as char);
            card_hand.push(drawn_card);
        }
        else{
            let drawn_card = format!("
┌──────────────┐
|{}             |
|              |
|              |
|              |
|       {}      |
|              |
|              |
|              |
|             {}|
└──────────────┘", face, spade, face);
            card_hand.push(drawn_card);
        }
    }


    for i in 0..=4{
        card_hand[i] = card_hand[i].replace("\n", "");  //176 chars per card
        let mut new_holder = CharHolder::new();

        for char in card_hand[i].chars(){
            new_holder.holder.push(char);
        }
        char_holder.push(new_holder);
    }
    //print!("{:?}", char_holder);


    for row in 0..11 {
        for card in 0..5{
            for char in (row * 16)..(row * 16) + 16{
                print!("{}", char_holder[card].holder[char]);
            }
        }
        println!("");
    }
}

fn evaluate_hand(poker_hand: &Hand) -> &str{
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

    value_tracker[13] = value_tracker[0]; //We need to add aces to the highest value count aswell since we only count them as lowest value when counting in previous for loop.
    //println!("Values: {:?}", value_tracker);
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
            if value_tracker[i] == 0 { // if element at index i is 0...
                if jokers_left == 0 { // ...then we check if we have jokers left to use.
                    break; // If not then we break out.
                }
                jokers_left -= 1; //Remove joker if used
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


    //filter out values that are 0
    // iterate over the vector, enumerate (index, value), but only iterate over 14 first indexes (else we enumarete jokers also)
    // filter out any enumeration that had a value of 0 and collect them into a vector.
    let mut values_filtered = value_tracker.into_iter().enumerate().take(13).filter(|&x|x.1 > 0).collect::<Vec<_>>();
    //sort by quantity first, then by value
    values_filtered.sort_unstable_by(|a, b| if b.1 == a.1 { //if same quantity
                                                                                (b.0).cmp(&a.0) //Sort by value
                                                                            } else { 
                                                                                (b.1).cmp(&a.1)}); //else sort by quantity
                                                                                
    //println!("{:?}", values_filtered);
    values_filtered[0].1 += jokers; //
    if values_filtered.len() == 1 {
    values_filtered.push((0, 0));
    }
    //println!("{:?}", values_filtered);
 
    match (is_flush, is_straight, values_filtered[0].1, values_filtered[1].1){
        (_,_,5,_) => "five-of-a-kind",
        (true, true, _, _) => if vec_pointer == 8 {"royal-flush"} else {"straight-flush"}, // if a joker was used then its only a straight flush
        (_,_,4,_) => "four-of-a-kind",
        (_,_,3,2) => "full-house",
        (true,_,_,_) => "flush",
        (_,true,_,_) => "straight",
        (_,_,3,_) => "three-of-a-kind",
        (_,_,2,2) => "two-pair",
        (_,_,2,_) => "one-pair",
        _ => "high-card"
     }
}


fn main() {
    let mut deck_of_cards = Deck::get_deck();
    let mut five_card_hand = Hand::draw_five_card_hand(&mut deck_of_cards);
    println!("{}", five_card_hand.hand_vec[0].suit as u8);

    //println!("{:?}", deck_of_cards);
    //println!("{:?}", five_card_hand);
    // let Deck {
    //     deck_vec: deck_destructred
    // } = &deck_of_cards;

    //println!("{}", deck_destructred.len());

    //print_hand(&five_card_hand);
    five_card_hand.discard_card_from_hand(0);
    println!("");
    //print_hand(&five_card_hand);
    five_card_hand.draw_until_five_cards(&mut deck_of_cards);
    println!("");
    print_hand(&five_card_hand);

    // let Deck {
    //     deck_vec: deck_destructred
    // } = &deck_of_cards;
    // println!("{}", deck_destructred.len());
    //five_card_hand.draw_until_five_cards(&mut deck_of_cards);

    //println!("{}", evaluate_hand(&five_card_hand));
    // let mut test_hand = Hand::new();
    // test_hand.hand_vec.push(Card::get_card(1, 2));
    // test_hand.hand_vec.push(Card::get_card(8, 1));
    // test_hand.hand_vec.push(Card::get_card(10, 1));
    // test_hand.hand_vec.push(Card::get_card(0, 2));
    // test_hand.hand_vec.push(Card::get_card(0, 3));
    //test_hand.hand_vec.push(Card::get_card(14, -1));
    // print_hand(&test_hand);
    // println!("Result: {}", evaluate_hand(&test_hand));
}