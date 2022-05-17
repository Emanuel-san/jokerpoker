use crate::card::*;
use crate::deck::*;

///Hand data type
#[derive(Debug)]
pub struct Hand {
    pub hand_vec: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            hand_vec: Vec::new(),
        }
    }
    //Draws until a Hand contains 5 cards
    pub fn draw_until_five_cards(&mut self, mut deck_of_cards: &mut Deck) {
        while self.hand_vec.len() < 5 {
            self.draw_card(&mut deck_of_cards)
        }
        assert!(
            self.hand_vec.len() == 5,
            "Hand::draw_until_five_cards:: Vector length is not 5, exit occured with length {}",
            self.hand_vec.len()
        );
    }
    ///Draws a card from the Deck
    pub fn draw_card(&mut self, deck_of_cards: &mut Deck) {
        match deck_of_cards.deck_vec.pop() {
            Some(card) => self.hand_vec.push(card),
            None => panic!("Deck::draw_card_from_deck: deck wasn't popped"),
        }
    }
    ///Discards any card that's not selected
    pub fn discard_selected(&mut self) {
        let mut discards_indexed = Vec::new();

        for (i, card) in self.hand_vec.iter().enumerate() { //enumarate each card
            if card.selected == false {
                discards_indexed.push(i.clone()); //if a card is not selected then clone the enumerated count into a vector
            }
        }

        for index in discards_indexed.iter().rev() { //iterate over the vector in reverse...
            self.hand_vec.remove(*index); //...and remove none-selected cards by their enumarated index
        }
    }

    //hard coded test hand
    pub fn _test_hand(&mut self){
        self.hand_vec.push(Card::get_card(13, 1));
        self.hand_vec.push(Card::get_card(13, 2));
        self.hand_vec.push(Card::get_card(1, 2));
        self.hand_vec.push(Card::get_card(4, 3));
        self.hand_vec.push(Card::get_card(9, 4));
    }
}
