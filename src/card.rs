use bevy::prelude::*;


#[derive(Debug, Component)]
pub struct Card{
    pub value: usize,
    pub suit: CardSuit,
}
#[derive(Debug, PartialEq, Component, Clone, Copy)]
pub enum CardSuit{
    Diamond = 1,
    Clove,
    Spade,
    Heart,
    Joker = 0
}

impl Card {

    //constructor
    fn new() -> Self {
        let value: usize = 0;
        let suit = CardSuit::Joker;
        Self {
            value,
            suit
        }
    }


    pub fn get_card(card_value: usize, suit_value: i8) -> Card{
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

