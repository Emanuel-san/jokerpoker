use crate::card::*;
use crate::deck::*;

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

    pub fn draw_card(&mut self, deck_of_cards: &mut Deck) {
        match deck_of_cards.deck_vec.pop() {
            Some(card) => self.hand_vec.push(card),
            None => panic!("Deck::draw_card_from_deck: Received a None option"),
        }
    }

    pub fn discard_selected(&mut self) {
        let mut discards_indexed = Vec::new();

        for (i, card) in self.hand_vec.iter().enumerate() {
            if card.selected == false {
                discards_indexed.push(i.clone());
            }
        }
        discards_indexed.reverse();

        for index in discards_indexed {
            self.hand_vec.remove(index);
        }
    }
}
