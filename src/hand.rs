//mod card;
use crate::card::*;

#[derive(Debug)]
pub struct Hand {
    pub hand_vec: Vec<Card>,
}

// impl IntoIterator for Hand{
//     type Item = Card;

//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.hand_vec.into_iter()
//     }
// }

impl Hand {

    /*Constructor */
    pub fn new() -> Self {
        let hand_of_cards = Vec::new();

        Self {
            hand_vec: hand_of_cards
        }
    }

    pub fn chk_duplicate(&self, new_card: &Card) -> bool{
        let Hand {
            hand_vec: temp
        } = &self;
        for card in temp{
            if card.value == new_card.value && card.suit == new_card.suit { // check if value and suit is the same
                return true
            }
        }
        return false
    }

    pub fn push_card_to_hand(&mut self){
        let new_card = Card::new();
        if self.chk_duplicate(&new_card)== false{ //check if the card already exist in a vec
            self.hand_vec.push(new_card); //push to vec if its not a duplicate
        }   
    }
}

