use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::CardSheet;
use crate::card::*;
use crate::deck::*;

pub struct HandPlugin;

#[derive(Component)]
pub struct HandState {
    pub hand_vec: Vec<Card>,
}



impl Plugin for HandPlugin {

    fn build(&self, app: &mut App){
        let hand_res = HandState {
            hand_vec: Vec::new()
        };

        app
        .insert_resource(hand_res)
        .add_system(draw_until_five_cards)
        .add_system(spawn_hand);
    }
}

fn spawn_hand(mut commands: Commands, cards: Res<CardSheet>, mut hand: ResMut<HandState>){
    
    let sprite = TextureAtlasSprite::new(1);
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: cards.0.clone(),
        transform: Transform::from_xyz(0.0,0.0,0.0),
        ..Default::default()
    }).id();
}

pub fn draw_until_five_cards(mut deck_of_cards: ResMut<DeckState>, mut hand: ResMut<HandState>){
    while hand.hand_vec.len() < 5{
        let drawn_card = deck_of_cards.deck_vec.pop();
        match drawn_card {
            Some(card) => hand.hand_vec.push(card),
            None => panic!("Deck::draw_card_from_deck: Received a None option")
        }
    }
    assert!(hand.hand_vec.len() <= 5, "Hand::draw_until_five_cards:: Can not exit with a vector length greater then 5, exit occured with length {}", hand.hand_vec.len());
}

impl HandState {

    fn new() -> Self {
        let hand_of_cards = Vec::new();

        Self {
            hand_vec: hand_of_cards
        }
    }



    // pub fn draw_five_card_hand(deck_of_cards: &mut DeckState) -> HandState{
    //     let mut new_hand = HandState::new();
    //     new_hand.draw_until_five_cards(deck_of_cards);
    //     new_hand
    // }

    pub fn discard_card_from_hand(&mut self, index: usize){
        if self.hand_vec.len() - 1 >= index {
            self.hand_vec.remove(index);
        } else {
            panic!("Hand::discard_card_from_hand: Index for removal is {} but vector length is {}", index, self.hand_vec.len());
        }
    }
    }

