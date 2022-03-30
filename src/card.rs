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

    pub fn new(value: u8, suit: u8) -> Self {
        Self {
            value,
            suit: match suit {
                1 => CardSuit::Diamond,
                2 => CardSuit::Clove,
                3 => CardSuit::Spade,
                4 => CardSuit::Heart,
                _ => panic!("Card:: Failed choosing random suit")
            }
        }
    }

    pub fn get_card() -> Card{
        let mut rng = thread_rng();
        let card_value = rng.gen_range(1..=14);
        let suit_value = rng.gen_range(1..=4);
        let new_card = Card::new(card_value, suit_value);
        new_card
    }
}
