use crate::card::*;
use bevy::prelude::*;
use rand::prelude::*;

pub struct DeckPlugin;

#[derive(Debug, Default)]
pub struct DeckState{
    pub deck_vec: Vec<Card>,
}

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App){
        let deck_res = DeckState {
            deck_vec: Vec::new()
        };
        app.insert_resource(deck_res)
            .add_startup_system(get_deck);

    }
}

/// Generates 52 cards into a deck with 2 jokers
/// # Panics
/// Function panics if the vector length its trying to return is not 54
fn get_deck(mut commands: Commands, mut deck_init: ResMut<DeckState>) {
    let mut rng = thread_rng();
    for suit in 0..=3{ //Generate 4 suits
        for card_face in 1..=13{ //13 card faces for each suit (Ace-King)
            deck_init.deck_vec.push(Card::get_card(card_face, suit));
        }
    }
    deck_init.deck_vec.push(Card::get_card(14, -1)); //Allow 2 Jokers
    deck_init.deck_vec.push(Card::get_card(14, -1));
    assert_eq!(deck_init.deck_vec.len(), 54, "Deck::generate_deck Vector length has to equal 54, current length is {}", deck_init.deck_vec.len()); //Make sure we have 54 cards (52 + two Jokers)
    deck_init.deck_vec.shuffle(&mut rng); //shuffle deck
    //println!("{:?}", deck_init.deck_vec);
    //commands.insert_resource(deck_init);
}

impl DeckState {

    ///Deck constructor
    fn new() -> Self{
        let deck_of_cards = Vec::new();

        Self{
            deck_vec: deck_of_cards,
        }
    }
}

