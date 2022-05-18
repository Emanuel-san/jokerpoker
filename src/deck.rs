use crate::card::*;
use rand::prelude::*;

///Deck data type
#[derive(Debug)]
pub struct Deck {
    pub deck_vec: Vec<Card>,
}

impl Deck {
    /// Generates 52 cards into a deck with 2 jokers
    /// # Panics
    /// Function panics if the vector length its trying to return is not 54
    pub fn new_deck() -> Self {
        let mut rng = thread_rng();
        let mut deck_init = Vec::new();
        for suit in 1..=4 { //Generate 4 suits
            for card_face in 1..=13 { //13 card faces for each suit (Ace-King)
                deck_init.push(Card::get_card(card_face, suit));
            }
        }
        deck_init.push(Card::get_card(14, 0)); //Push 2 jokers to the deck
        deck_init.push(Card::get_card(14, 0));
        assert_eq!(deck_init.len(), 54, "Deck::generate_deck Vector length has to equal 54, current length is {}", deck_init.len()); //Make sure we have 54 cards (52 + two Jokers)
        deck_init.shuffle(&mut rng); //shuffle deck

        Self {
            deck_vec: deck_init
        }
    }
}
