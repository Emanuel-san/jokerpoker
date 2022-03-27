//Should i separate Card object from Deck object in different files?
#[derive(Debug)]
pub struct Card{
    value: u8,
    suit: CardSuit,
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
}