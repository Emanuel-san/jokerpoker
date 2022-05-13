use crate::card::*;
use crate::hand::*;
use crate::utils::*;
///Holds current credits left in play
pub struct Wallet {
    pub credits: usize,
}
///Holds the evaluated values of a Hand
pub struct Evaluation<'a> {
    pub hand_value: usize,
    hand_type: &'a str, //string slice will live as long as lifetime 'a (Evaluation)
}

#[derive(PartialEq)]
pub enum MachineState {
    CoinsAvailable,
    CardSelection,
    InsertCoin,
    Double,
    Win,
}

impl <'a> Evaluation<'a> {
    pub fn new(hand_value: usize, hand_type: &'a str) -> Self {
        Self {
            hand_value,
            hand_type,
        }
    }
    ///Check if the evaluation was a winning hand
    pub fn chk_evaluation_for_win(&self, state: &mut MachineState) {
        if self.hand_value > 0 {
            println!("YOU WON!\n{} pays {}", self.hand_type, self.hand_value);
            *state = MachineState::Win;
        } else {
            println!("No win");
        }
    }
}

impl Wallet {
    pub fn new() -> Self {
        Self { credits: 0 }
    }
    ///Check if there are credits left
    pub fn chk_funds(&mut self, state: &mut MachineState) {
        if self.credits == 0 {
            *state = MachineState::InsertCoin;
            print_insert_coin();
        }
    }
    ///Reduce credits by 1
    pub fn reduce_funds(&mut self) {
        self.credits -= 1;
    }
    ///Add to playable credits
    pub fn add_funds(&mut self, funds_to_add: &usize) {
        self.credits += funds_to_add;
    }
}

//Evaluate if the machine or the player won the doubling Hand.
pub fn evaluate_doubling(
    hand: &Hand, 
    credits_won: &mut usize, 
    selected_index: &usize, 
    state: &mut MachineState,)
    {
    if hand.hand_vec[0].value < hand.hand_vec[*selected_index].value{ //If the Card value selected by the player is higher then the machine, then player wins
        *credits_won *= 2; //Double the current credits won by 2
        println!("You beat the dealer! Credits won are now {}", credits_won);
        *state = MachineState::Win; //Set the state back to Win in case the player want's to double again.
    } 
    else {
        println!("BUST!");
        *state = MachineState::CoinsAvailable;
    }
}

///Evaluate a Hand and check if it's a winning hand
pub fn evaluate_hand(poker_hand: &Hand) -> Evaluation {
    let mut suit_tracker = [0u8; 4];
    let mut value_tracker = [0u8; 15];
    let mut jokers: u8 = 0;

    for card in &poker_hand.hand_vec { //Check each card and hand value and suit to corresponding array and element
        value_tracker[card.value as usize] += 1;
        match card.suit {
            CardSuit::Diamond => suit_tracker[0] += 1,
            CardSuit::Clove => suit_tracker[1] += 1,
            CardSuit::Spade => suit_tracker[2] += 1,
            CardSuit::Heart => suit_tracker[3] += 1,
            CardSuit::Joker => jokers += 1,
        }
    }

    value_tracker[0] = value_tracker[13]; //We need to add aces to the lowest value count aswell

    //filter out the suit_tracker where 0 values occur and collect the none zero values into a new vec (counted_suits)...
    let mut counted_suits = suit_tracker
        .into_iter()
        .filter(|&x| x > 0)
        .collect::<Vec<_>>();
    counted_suits.sort_unstable(); //... and sort it.
    counted_suits[0] += jokers; // add joker(s) to the index 0 incase it creates a flush. (i.e counted_suits[0] == 3 and we have 2 jokers, then we have a flush.)
    let is_flush = counted_suits[0] == 5; // If index 0 contains element 5 then we have a flush.
    let mut is_straight = false;

    //We use a "pointer" to avoid looking at cards again that was a part of another sequence.
    //This is only for checking if we have a poker hand with a straight.
    let mut pointer = 14;
    while pointer > 3 {                      // If the pointer reachs value 3 its no point at looking at the rest since Ace-2-3-4 will not make a straight.
        let mut jokers_left = jokers;    // Each time a sequence "fails" we re-declare jokers to use in the next sequence
        let mut straight_cards = 0;     // Reset to 0 on "failed" sequence.
        for i in (0..pointer).rev() { // loop through pointer in reverse 13..0
            if value_tracker[i] == 0 {       // if element at index is 0...
                if jokers_left == 0 {        // ...then we check if we have jokers left to use.
                    break;                   // If not then we break out.
                }
                jokers_left -= 1;            //Remove joker if used
            } else if i == pointer - 1 {     //Since we use jokers we will only reduce pointer if this is true, else its possible we miss a possible straight in the next sequence
                pointer -= 1;
            }
            straight_cards += 1;             //if we found a index with a none zero value or we used a joker, add 1 straight card.
        }
        pointer -= 1;                        //always reduce by atleast one since even tho we used jokers we can guarantee the first index in the sequence we check will not form a straight.
        if straight_cards == 5 {
            is_straight = true;              // We have a straight!
            break;
        }
    }
    value_tracker[0] = 0; // reset lowest index to 0 to not count aces twice
    //filter out values that are 0
    // iterate over the vector, enumerate (index, value), but only iterate over 14 first indexes (else we enumarete jokers also)
    // filter out any enumeration that had a value of 0 and collect them into a vector.
    let mut values_filtered = value_tracker
        .into_iter()
        .enumerate()
        .take(14)
        .filter(|&x| x.1 > 0)
        .collect::<Vec<_>>();
    //sort by quantity first, then by value
    values_filtered.sort_unstable_by(|a, b| {
        if b.1 == a.1 { //if same quantity
            (b.0).cmp(&a.0) //Sort by value
        } 
        else {//else sort by quantity
            (b.1).cmp(&a.1)
        }
    });

    values_filtered[0].1 += jokers; //add jokers to the highest sorted quantity

    //push a tuple to the vector if lenth is 1 to satisfy the match expression below, i.e five a kind leave us with [(5, x)] and would panic since we are then
    //looking outside the vector when matching value at values_filtered[1].1
    if values_filtered.len() == 1 { 
        values_filtered.push((0, 0));
    }
    match (
        is_flush,
        is_straight,
        values_filtered[0].1,
        values_filtered[1].1,
    ) {
        (_, _, 5, _) => Evaluation::new(100, "Five Of A Kind"),
        (true, true, _, _) => {
                        if pointer == 8 {
                             Evaluation::new(250, "Royal Flush")
                            } 
                        else {
                            Evaluation::new(50, "Straight Flush") // if a joker was used then its only a straight flush
                        }
                    }
        (_, _, 4, _) => Evaluation::new(25, "Four Of A Kind"),
        (_, _, 3, 2) => Evaluation::new(9, "Full House"),
        (true, _, _, _) => Evaluation::new(6, "Flush"),
        (_, true, _, _) => Evaluation::new(4, "Straight"),
        (_, _, 3, _) => Evaluation::new(3, "Three Of A Kind"),
        (_, _, 2, 2) => Evaluation::new(2, "Two Pair"),
        (_, _, 2, _) => {
                        if values_filtered[0].0 >= 10 {
                            Evaluation::new(1, "Jacks Or Better")
                        } 
                        else {
                            Evaluation::new(0, "")
                        }
                    }
        _ => Evaluation::new(0, ""),
    }
}
