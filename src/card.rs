
#[derive(Debug)]
pub struct Card{
    pub value: u8,
    pub suit: CardSuit,
}
#[derive(Debug, PartialEq, Clone, Copy)]
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
        let value: u8 = 0;
        let suit = CardSuit::Joker;
        Self {
            value,
            suit
        }
    }


    pub fn get_card(card_value: u8, suit_value: u8) -> Card{
        let mut new_card = Card::new();
        new_card.value = card_value;
        new_card.suit = match suit_value {
            0 => CardSuit::Joker,
            1 => CardSuit::Diamond,
            2 => CardSuit::Clove,
            3 => CardSuit::Spade,
            4 => CardSuit::Heart,
            _ => panic!("Card::generate_card: Failed choosing random suit")
        };
        new_card
    }
}

