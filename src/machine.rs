use clearscreen::ClearScreen;

use crate::hand::*;
use crate::card::*;
use crate::input::*;
use crate::utils::*;

pub struct Funds{
    pub credits: usize,
}

pub struct Evaluation {
    hand_value: usize,
    hand_type: String,
}

#[derive(PartialEq)]
pub enum MachineState{
    CoinsAvailable,
    CardSelection,
    InsertCoin,
}

impl Evaluation {
    pub fn new(hand_value: usize, hand_type: String) -> Self {

        Self {
            hand_value,
            hand_type
        }
    }

    pub fn print_and_add_win(&self, funds: &mut Funds){
        if self.hand_value > 0 {
            println!("YOU WON!\n{} pays {}", self.hand_type, self.hand_value);
            funds.credits += self.hand_value;
        }
    }
}


impl Funds {
    pub fn new() -> Self {

        Self {
            credits: 0,

        }
    }
    pub fn chk_funds(&mut self, state: &mut MachineState){
        if self.credits == 0{
            ClearScreen::default().clear().expect("failed to clear terminal");
            *state = MachineState::InsertCoin;
            print_insert_coin();
        }
    }
    pub fn reduce_funds(&mut self){
        self.credits -= 1;
    }

    pub fn add_funds(input: &UserInput ,funds: &mut Funds, state: &mut MachineState) {
        if let Ok(()) = input.chk_draw_input(){
            if funds.credits > 0{
                ClearScreen::default().clear().expect("failed to clear terminal");
                *state = MachineState::FundsAvailable;
            } else {
                println!("No available funds");
            }
        } else {
            if let Ok(input) = input.chk_funds_input(){
                ClearScreen::default().clear().expect("failed to clear terminal");
                print_insert_coin();
                funds.credits += input;
            } else {
                println!("Invalid input");
            }
        }
        
    }
}


pub fn card_selection(input: &UserInput, hand: &mut Hand, holder: &mut Vec<CharHolder>, state: &mut MachineState, funds: &Funds){
    if let Ok(()) = input.chk_draw_input(){
        ClearScreen::default().clear().expect("failed to clear terminal");
        *state = MachineState::FundsAvailable;
    } else {
        if let Ok(parsed_input) = input.chk_select_input(){
            ClearScreen::default().clear().expect("failed to clear terminal");
            let card: &mut Card = &mut hand.hand_vec[parsed_input - 1];
            card.alter_selection();
            *holder = format_hand(&hand);
            println!("Funds: {}", funds.credits);
            print_hand(&holder);
        } else {
            println!("Invalid input");
        }
    }
}

pub fn evaluate_hand(poker_hand: &Hand) -> Evaluation{
    let mut suit_tracker = [0u8; 4];
    let mut value_tracker = [0u8; 15];
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

    //filter out the suit_tracker where 0 values occur and collect the none zero values into a new vec (counted_suits)...
    let mut counted_suits = suit_tracker.into_iter()
    .filter(|&x| x > 0).collect::<Vec<_>>();
    counted_suits.sort_unstable();//... and sort it.
    counted_suits[0] += jokers; // add jokers to the index 0 incase they create a flush. (i.e index == 3 and we have 2 jokers, then we have a flush.)
    let is_flush = counted_suits[0] == 5; // If index 0 contains element 5 then we have a flush.
    let mut is_straight = false;

    let mut pointer = 14;
    //We use a "pointer" to avoid looking at cards again that was a part of another sequence.
    //This is only for checking if we have a poker hand with a straight.

    while pointer > 3{// If the pointer reachs value 3 its no point at looking at the rest since Ace-2-3-4 will not make a straight. 
        let mut jokers_left = jokers; //Each time a sequence "fails" we re-declare jokers to use in the next sequence
        let mut straight_cards = 0; // Reset to 0 on "failed" sequence.
        for i in (0..pointer).rev(){ // represents the sequence (0..ptr) = 0-13
            if value_tracker[i] == 0 { // if element at index is 0...
                if jokers_left == 0 { // ...then we check if we have jokers left to use.
                    break; // If not then we break out.
                }
                jokers_left -= 1; //Remove joker if used
            }
            else if i==pointer-1 { //Since we use jokers we will only reduce pointer if this is true, else its possible we miss a possible straight in the next sequence
                pointer -= 1;
            }
            straight_cards += 1; //if we found a index with a none zero value or we used a joker, add 1 straight card.
        }
        pointer -= 1; //always reduce by atleast one since even tho we used jokers we can guarantee the first index we check will not form a straight.
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
                                                                                
    values_filtered[0].1 += jokers; //
    if values_filtered.len() == 1 {
    values_filtered.push((0, 0));
    }
    let new_evaluation = match (is_flush, is_straight, values_filtered[0].1, values_filtered[1].1){
        (_,_,5,_) => Evaluation::new(100, String::from("Five Of A Kind")),
        (true, true, _, _) => 
            if vec_pointer == 8 {
                Evaluation::new(250, String::from("Royal Flush"))
            } else {
                Evaluation::new(50, String::from("Straight Flush"))// if a joker was used then its only a straight flush
            },
        (_,_,4,_) => Evaluation::new(25, String::from("Four Of A Kind")),
        (_,_,3,2) => Evaluation::new(9, String::from("Full House")),
        (true,_,_,_) => Evaluation::new(6, String::from("Flush")),
        (_,true,_,_) => Evaluation::new(4, String::from("Straight")),
        (_,_,3,_) => Evaluation::new(3, String::from("Three Of A Kind")),
        (_,_,2,2) => Evaluation::new(2, String::from("Two Pair")),
        (_,_,2,_) => Evaluation::new(1, String::from("Jacks Or Better")),
        _ => Evaluation::new(0, String::from("High Card"))
    };
    new_evaluation
}