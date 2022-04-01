use crate::card::*;

#[derive(Debug)]
pub struct Deck{
    pub deck: Vec<Card>,
}

impl Deck {

    fn new() -> Self{
        let deck_of_cards = Vec::new();

        Self{
            deck: deck_of_cards,
        }
    }

    pub fn generate_deck() -> Deck {
        let mut deck_init = Deck::new();
        for suit in 0..=3{ //Generate 4 suits
            for card_face in 0..=12{ //13 card faces for each suit (Ace-King)
                deck_init.deck.push(Card::generate_card(card_face, suit));
            }
        }
        deck_init.deck.push(Card::generate_card(14, -1)); //Allow 2 Jokers
        deck_init.deck.push(Card::generate_card(14, -1));
        deck_init
    }
}

