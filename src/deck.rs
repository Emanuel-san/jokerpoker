use crate::card::*;
use rand::prelude::*;

#[derive(Debug)]
pub struct Deck{
    pub deck_vec: Vec<Card>,
}

impl Deck {

    ///Deck constructor
    fn new() -> Self{
        let deck_of_cards = Vec::new();

        Self{
            deck_vec: deck_of_cards,
        }
    }
    /// Generates 52 cards into a deck with 2 jokers
    /// # Panics
    /// Function panics if the vector length its trying to return is not 54
    pub fn get_deck() -> Deck {
        let mut rng = thread_rng();
        let mut deck_init = Deck::new();
        for suit in 0..=3{ //Generate 4 suits
            for card_face in 0..=12{ //13 card faces for each suit (Ace-King)
                deck_init.deck_vec.push(Card::get_card(card_face, suit));
            }
        }
        deck_init.deck_vec.push(Card::get_card(14, -1)); //Allow 2 Jokers
        deck_init.deck_vec.push(Card::get_card(14, -1));
        assert_eq!(deck_init.deck_vec.len(), 54, "Deck::generate_deck Vector length has to equal 54, current length is {}", deck_init.deck_vec.len()); //Make sure we have 54 cards (52 + two Jokers)
        deck_init.deck_vec.shuffle(&mut rng); //shuffle deck
        deck_init
    }
}

