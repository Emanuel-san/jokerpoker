//mod card;
use crate::card::Card;
use rand::prelude::*;

#[derive(Debug)]
pub struct Hand {
    pub hand_vec: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut hand_of_cards = Vec::new();
        let mut card_value: u8;
        let mut suit_value: u8;
        let mut new_card: Card;
        
        while hand_of_cards.len() < 5 {
            card_value = rng.gen_range(1..=14);
            suit_value = rng.gen_range(1..=4);
            new_card = Card::new(card_value, suit_value);
            if Hand::chk_duplicate(&hand_of_cards, &new_card)
                {hand_of_cards.push(new_card);}
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

