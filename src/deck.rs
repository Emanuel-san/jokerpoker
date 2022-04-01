use crate::card::*;


pub struct Deck{
    deck: Vec<Card>,
}

impl Deck {

    fn new() -> Self{
        let mut deck_of_cards = Vec::new();

        Self{
            deck: deck_of_cards,
        }
    }
}

