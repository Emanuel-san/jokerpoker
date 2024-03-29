///Card data type
#[derive(Debug)]
pub struct Card {
    pub value: u8,
    pub suit: CardSuit,
    pub selected: bool,
}

///enum for named card suits
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CardSuit {
    Diamond = 1,
    Clove,
    Spade,
    Heart,
    Joker,
}

///convert bytes to u8 integer
fn convert_bytes_to_integer(bytes: &[u8]) -> u8 {
    u8::from_be_bytes(bytes.try_into().unwrap())
}

impl Card {
    ///constructor
    fn new() -> Self {
        Self {
            value: 0,
            suit: CardSuit::Joker,
            selected: false,
        }
    }
    ///create a card from given face and suit values
    pub fn get_card(card_value: u8, suit_value: u8) -> Card {
        let mut new_card = Card::new();
        new_card.value = card_value;
        new_card.suit = match suit_value {
            0 => CardSuit::Joker,
            1 => CardSuit::Diamond,
            2 => CardSuit::Clove,
            3 => CardSuit::Spade,
            4 => CardSuit::Heart,
            _ => panic!("Card::generate_card: Failed matching suit"),
        };
        new_card
    }

    //returns each face in bytes
    pub fn get_face_bytes(&self) -> Vec<u8> {
        let face: Vec<u8>;
        match self.value {
            14 => face = vec![74, 75],
            12 => face = vec![75, 32],
            11 => face = vec![81, 32],
            10 => face = vec![74, 32],
            9 => face = vec![49, 48],
            13 => face = vec![65, 32],
            number => {
                face = vec![
                    convert_bytes_to_integer((number + 1).to_string().as_bytes()),
                    32,
                ] // face values between 2-10
            }
        }
        face
    }

    // returns each suit in bytes
    pub fn get_suit_bytes(&self) -> Vec<u8> {
        let suit: Vec<u8>;
        match self.suit {
            CardSuit::Joker => suit = vec![74],
            CardSuit::Diamond => suit = vec![226, 153, 162],
            CardSuit::Spade => suit = vec![226, 153, 160],
            CardSuit::Clove => suit = vec![226, 153, 163],
            CardSuit::Heart => suit = vec![226, 153, 161],
        }
        suit
    }
}
