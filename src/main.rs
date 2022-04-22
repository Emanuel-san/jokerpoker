mod card;
mod hand;
mod deck;
mod debug;

use bevy::prelude::*;
use hand::*;
use deck::*;
use card::*;
use debug::DebugPlugin;

const CLEAR: Color = Color::DARK_GREEN;
const NORMAL_BUTTON: Color = Color::MIDNIGHT_BLUE;
//const BUTTON_PRESSED: Color = Color::CYAN;
pub struct CardSheet(Handle<TextureAtlas>);



fn main() {
 
    App::new()
    .insert_resource(ClearColor(CLEAR))
    .insert_resource(WindowDescriptor {
        width: 1600.0,
        height: 900.0,
        title: "Five Card Poker".to_string(),
        vsync: true,
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(DebugPlugin)
    .add_startup_system_to_stage(StartupStage::PreStartup, load_cards)
    .add_startup_system(setup)
    .add_startup_system(spawn_deck_sprite)
    .add_plugin(DeckPlugin)
    .add_plugin(HandPlugin)
    .run();

}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(150.0)),
                // position button
                margin: Rect { left: Val::Px(1300.0), bottom: Val::Px(50.0), ..Default::default()},
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            transform: Transform::from_xyz(0.0,0.0,0.0),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Deal",
                        TextStyle {
                            font: asset_server.load("PlayfairDisplay-Regular.ttf"),
                            font_size: 70.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            });
        }

fn load_cards(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let image = asset_server.load("faces.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image, 
        Vec2::new(168.0, 245.0), 
        13, 
        5,
        Vec2::new(-0.5, -2.0));
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(CardSheet(atlas_handle));
}

fn spawn_deck_sprite (mut commands: Commands, cards: Res<CardSheet>){
    let sprite = TextureAtlasSprite::new(54);
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: cards.0.clone(),
        transform: Transform::from_xyz(-700.0,300.0,0.0),
        ..Default::default()
    }).id();
}

fn print_hand(hand_obj: &HandState) {

    for card in &hand_obj.hand_vec{
        match card.value {
            13 | 14 => (),
            12 => print!("King of "),
            11 => print!("Queen of "),
            10 => print!("Jack of "),
            0 => print!("Ace of "),
            number => print!("{} of ", number + 1),
        };
        match card.suit {
            CardSuit::Joker => print!("Joker | "),
            CardSuit::Diamond => print!("Diamonds | "),
            CardSuit::Spade => print!("Spades | "),
            CardSuit::Clove => print!("Cloves | "),
            CardSuit::Heart => print!("Hearts | "),
        }
    }
    println!("");
}

fn evaluate_hand(poker_hand: &HandState) -> &str{
    let mut suit_tracker = vec![0u8; 4];
    let mut value_tracker = vec![0u8; 15];
    let mut jokers: u8 = 0;

    for card in &poker_hand.hand_vec{
        value_tracker[card.value as usize] += 1;
        match card.suit {
            CardSuit::Diamond => suit_tracker[0] += 1,
            CardSuit::Clove => suit_tracker[1] += 1,
            CardSuit::Spade => suit_tracker[2] += 1,
            CardSuit::Heart => suit_tracker[3] += 1,
            CardSuit::Joker => jokers +=1
        }
    }

    value_tracker[13] = value_tracker[0]; //We need to add aces to the highest value count aswell since we only count them as lowest value when counting in previous for loop.
    //println!("Values: {:?}", value_tracker);
    // println!("Suits: {:?}", suit_counter);
    // println!("Jokers: {}", jokers);

    //filter out the suit_tracker where 0 values occur and collect the none zero values into a new vec (counted_suits)...
    let mut counted_suits = suit_tracker.into_iter()
    .filter(|&x| x > 0).collect::<Vec<_>>();
    counted_suits.sort_unstable();//... and sort it.
    counted_suits[0] += jokers; // add jokers to the index 0 incase they create a flush. (i.e index == 3 and we have 2 jokers, then we have a flush.)
    //println!("{:?}", counted_suits);
    let is_flush = counted_suits[0] == 5; // If index 0 contains element 5 then we have a flush.
    let mut is_straight = false;

    let mut vec_pointer = 14;
    //We use a "pointer" to avoid looking at cards again that was a part of another sequence.
    //This is only for checking if we have a poker hand with a straight.

    while vec_pointer > 3{// If the pointer reachs value 3 its no point at looking at the rest since Ace-2-3-4 will not make a straight. 
        let mut jokers_left = jokers; //Each time a sequence "fails" we will shadow this variable and re-declare jokers to use in the next sequence
        let mut straight_cards = 0; // Same as above though we reset to 0 on "failed" sequence.
        for i in (0..vec_pointer).rev(){ // represents the sequence (0..ptr) = 0-13
            if value_tracker[i] == 0 { // if element at index i is 0...
                if jokers_left == 0 { // ...then we check if we have jokers left to use.
                    break; // If not then we break out.
                }
                jokers_left -= 1; //Remove joker if used
            }
            else if i==vec_pointer-1 { //Since we use jokers we will only reduce vec_pointer if this is true, else its possible we miss a possible straight in the next sequence
                vec_pointer -= 1;
            }
            straight_cards += 1; //if we found a index with a none zero value or we used a joker, add 1 straight card.
        }
        vec_pointer -= 1; //always reduce by atleast one since even tho we used jokers we can guarantee the first index we check will not form a straight.
        if straight_cards == 5 { // We have a straight!
            is_straight = true;
            break;
        }
    }


    //filter out values that are 0
    // iterate over the vector, enumerate (index, value), but only iterate over 14 first indexes (else we enumarete jokers also)
    // filter out any enumeration that had a value of 0 and collect them into a vector.
    let mut values_filtered = value_tracker.into_iter().enumerate().take(13).filter(|&x|x.1 > 0).collect::<Vec<_>>();
    //sort by quantity first, then by value
    values_filtered.sort_unstable_by(|a, b| if b.1 == a.1 { //if same quantity
                                                                                (b.0).cmp(&a.0) //Sort by value
                                                                            } else { 
                                                                                (b.1).cmp(&a.1)}); //else sort by quantity
                                                                                
    //println!("{:?}", values_filtered);
    values_filtered[0].1 += jokers; //add jokers to the highest value 
    if values_filtered.len() == 1 { //
    values_filtered.push((0, 0));
    }
    //println!("{:?}", values_filtered);
 
    match (is_flush, is_straight, values_filtered[0].1, values_filtered[1].1){
        (_,_,5,_) => "five-of-a-kind",
        (true, true, _, _) => if vec_pointer == 8 {"royal-flush"} else {"straight-flush"}, // if a joker was used then its only a straight flush
        (_,_,4,_) => "four-of-a-kind",
        (_,_,3,2) => "full-house",
        (true,_,_,_) => "flush",
        (_,true,_,_) => "straight",
        (_,_,3,_) => "three-of-a-kind",
        (_,_,2,2) => "two-pair",
        (_,_,2,_) => "one-pair",
        _ => "high-card"
     }
}
