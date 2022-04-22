use bevy::prelude::*;


#[derive(Debug, Component)]
pub struct Card{
    pub value: i8,
    pub suit: CardSuit,
}
#[derive(Debug, PartialEq, Component)]
pub enum CardSuit{
    Diamond = 1,
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


    pub fn get_card(card_value: i8, suit_value: i8) -> Card{
        let mut new_card = Card::new();
        new_card.value = card_value;
        new_card.suit = match suit_value {
            -1 => CardSuit::Joker,
            0 => CardSuit::Diamond,
            1 => CardSuit::Clove,
            2 => CardSuit::Spade,
            3 => CardSuit::Heart,
            _ => panic!("Card::generate_card: Failed choosing random suit")
        };
        new_card
    }
}

