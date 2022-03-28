use std::num::ParseIntError;

#[derive(Debug)]
pub struct Card{
    pub value: u8,
    pub suit: CardSuit,
}
#[derive(Debug)]
pub enum CardSuit{
    Diamond,
    Clove,
    Spade,
    Heart
}

impl Card {

    pub fn new(value: u8, suit: CardSuit) -> Self {
        Self {
            value,
            suit
        }
    }

    pub fn random_suit(rng_value: u8) -> CardSuit{
        match rng_value {
            1 => CardSuit::Diamond,
            2 => CardSuit::Clove,
            3 => CardSuit::Spade,
            4 => CardSuit::Heart,
            _ => panic!("Card:: Failed choosing random suit")
        }
    }
}
