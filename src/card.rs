
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
}
