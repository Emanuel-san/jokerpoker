
use crate::card::*;
use crate::deck::*;

#[derive(Debug)]
pub struct Hand {
    pub hand_vec: Vec<Card>,
}


impl Hand {

    pub fn new() -> Self {
        let hand_of_cards = Vec::new();

        Self {
            hand_vec: hand_of_cards
        }
    }

    pub fn draw_until_five_cards(&mut self, deck_of_cards: &mut Deck){
        while self.hand_vec.len() < 5{
            let drawn_card = deck_of_cards.deck_vec.pop();
            match drawn_card {
                Some(card) => self.hand_vec.push(card),
                None => panic!("Deck::draw_card_from_deck: Received a None option")
            }
        }
        assert!(self.hand_vec.len() == 5, "Hand::draw_until_five_cards:: Vector length is not 5, exit occured with length {}", self.hand_vec.len());
    }


    pub fn discard_card_from_hand(&mut self, index: usize){
        if self.hand_vec.len() - 1 >= index {
            self.hand_vec.remove(index);
        } else {
            panic!("Hand::discard_card_from_hand: Index for removal is {} but vector length is {}", index, self.hand_vec.len());
        }
    }
}

