//mod card;
use crate::card::*;

#[derive(Debug)]
pub struct Hand {
    pub hand_vec: Vec<Card>,
}

impl Hand {
    
    /*Constructor */
    pub fn new() -> Self {
        let mut hand_of_cards = Vec::new();
        let mut new_card: Card;
        
        while hand_of_cards.len() < 5 { //loop untill vec holds 5 elements
            new_card = Card::get_card(); //get a card
            if Hand::chk_duplicate(&hand_of_cards, &new_card) == false{ //check if the card already exist in a vec
                hand_of_cards.push(new_card); //push to vec if its not a duplicate
            }
        }

        Self {
            hand_vec: hand_of_cards
        }
    }

    pub fn chk_duplicate(vec_of_cards: &Vec<Card>, new_card: &Card) -> bool{

        for card in vec_of_cards{
            if card.value == new_card.value && card.suit == new_card.suit {
                return true
            }
        }
        return false
    }
}

