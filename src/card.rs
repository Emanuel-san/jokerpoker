//Should i separate Card object from Deck object in different files?

pub struct Card{
    value: u8,
    suit: CardSuit,
}
pub enum CardSuit{
    Diamond,
    Clove,
    Spade,
    Heart
}