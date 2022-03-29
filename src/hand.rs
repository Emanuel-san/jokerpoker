//mod card;
use crate::card::Card;
use rand::prelude::*;

#[derive(Debug)]
pub struct Hand {
    deck_vec: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut hand_of_cards = Vec::new();
        
        while hand_of_cards.len() < 5 {
            let card_value = rng.gen_range(1..=14);
            let suit_value = rng.gen_range(1..=4);
            hand_of_cards.push(Card::new(card_value, suit_value));
        }

        Self {
            deck_vec: hand_of_cards
            }
        }
    }

