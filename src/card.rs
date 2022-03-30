use rand::prelude::*;

#[derive(Debug)]
pub struct Card{
    pub value: u8,
    pub suit: CardSuit,
}
#[derive(Debug, PartialEq)]
pub enum CardSuit{
    Diamond,
    Clove,
    Spade,
    Heart
}

impl Card {

    pub fn new() -> Self {
        let mut rng = thread_rng(); //declare a rng gen (rand dependency)
        let card_value = rng.gen_range(1..=14); //random card value
        let suit_value = rng.gen_range(1..=4); //random suit value

        Self {  //construct a card
            value: card_value,
            suit: match suit_value {
                1 => CardSuit::Diamond,
                2 => CardSuit::Clove,
                3 => CardSuit::Spade,
                4 => CardSuit::Heart,
                _ => panic!("Card:: Failed choosing random suit")
            }
        }
    }
}
