//use rand::prelude::*;

#[derive(Debug)]
pub struct Card{
    pub value: i8,
    pub suit: CardSuit,
}
#[derive(Debug, PartialEq)]
pub enum CardSuit{
    Diamond,
    Clove,
    Spade,
    Heart,
    Joker
}

impl Card {

    //constructor
    fn new() -> Self {
        let value: i8 = -1;
        let suit = CardSuit::Joker;
        Self {
            value,
            suit
        }
    }


    pub fn generate_card(card_value: i8, suit_value: i8) -> Card{
        let mut new_card = Card::new();
        new_card.value = card_value;
        new_card.suit = match suit_value {
            -1 => CardSuit::Joker,
            0 => CardSuit::Diamond,
            1 => CardSuit::Clove,
            2 => CardSuit::Spade,
            3 => CardSuit::Heart,
            _ => panic!("Card:: Failed choosing random suit")
        };
        new_card
    }
}

