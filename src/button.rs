use std::ops::DerefMut;
use bevy::color::Color;
use bevy::prelude::*;
use crate::{TurnCount, UpdateQueue};
use crate::game_handler;
use crate::types::{Board, Coordinates, Player};
pub fn button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Coordinates,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut turns: ResMut<TurnCount>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut queue: ResMut<UpdateQueue>,
) {
    for (interaction, mut color, mut border_color, coordinates) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::WHITE.into();
                // border_color.0 = Color::srgb(1.0,0.0,0.0).into();
                println!("X: {} Y: {}", coordinates.x, coordinates.y);
                println!("Turn: {}", turns.0);
                game_handler::handle_move((coordinates.x, coordinates.y), &mut turns, &mut board, &mut commands, &mut queue);
            }
            Interaction::Hovered => {
                *color = Color::BLACK.into();
                // if (board.unwrap()[position.1][position.0].owner == if turns.0 % 2 == 0 {Player::PLAYER1} else {Player::PLAYER2}) || turns.0 < 2 {
                //     border_color.0 = match turns.0 % 2 {
                //         0 => Color::srgb(1.0, 0.0, 0.0),
                //         1 => Color::srgb(0.0, 0.0, 1.0),
                //         _ => Color::BLACK};
                // }
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}

pub fn update_button_colour (
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Coordinates,
            &Children,
        ),
    >,
    mut updates: ResMut<UpdateQueue>,
    mut time: ResMut<Time>,
    mut board: ResMut<Board>,
    mut text_query: Query<&mut Text>,
) {
    for ref mut timer_data in updates.0.iter_mut() {
        println!("Updating button colour");
        let (timer, data) = timer_data;
        if timer.tick(time.delta()).just_finished() {
            for (_, _, mut border_color, coordinates, children) in &mut interaction_query {
                if *coordinates == data.coordinates {
                    let mut text = text_query.get_mut(children[0]).unwrap();
                    let mut new_board = board.clone();
                    if let Board::Board(ref mut inner_board) = board.as_mut() {
                        let mut new_board = new_board.unwrap();
                        new_board[coordinates.y][coordinates.x].changed = false;
                        *inner_board = new_board; // Modify the inner value
                    }
                    match board.unwrap()[coordinates.y][coordinates.x].owner {
                        Player::NONE => {
                            border_color.0 = Color::BLACK;
                            text.0 = String::from("");
                        }
                        Player::PLAYER1 => {
                            border_color.0 = Color::srgb(1.0, 0.0, 0.0);

                            text.0 = board.unwrap()[coordinates.y][coordinates.x].val.to_string();
                        }
                        Player::PLAYER2 => {
                            border_color.0 = Color::srgb(0.0, 0.0, 1.0);

                            text.0 = board.unwrap()[coordinates.y][coordinates.x].val.to_string();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
