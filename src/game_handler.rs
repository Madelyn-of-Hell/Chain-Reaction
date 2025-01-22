use crate::types::{Coordinates, Board, Player, Add_Queue_data};
use crate::types::Player::{PLAYER1, PLAYER2};
use std::process::{exit};
use bevy::prelude::*;
use std::io::Write;
use std::time::Duration;
use bevy::winit::UpdateMode;
use crate::{TurnCount, UpdateQueue};

pub fn handle_move(coordinates_raw: (usize, usize), turns: &mut ResMut<TurnCount>, board: &mut ResMut<Board>, commands: &mut Commands, queue: &mut ResMut<UpdateQueue>) -> u8 {
    // The game loop
    let mut turn:bool = turns.0 % 2 != 0;
    let player = match turn {
        false => PLAYER1,
        true => PLAYER2
    };
    print!("Player{}'s turn: ", turn as u8 + 1); std::io::stdout().flush().unwrap();

    let coordinates:Option<Coordinates> = Coordinates::new(coordinates_raw.0, coordinates_raw.1);

    if coordinates.is_some() {
        let coordinates = coordinates.unwrap();
            match board.add(
                player,
                coordinates,
                0,
                turns.0 < 2,
                commands,
                queue
            ) {
                Ok(new_board) => {
                    turn = !turn;
                    turns.0 += 1;
                    if new_board.gameover() != Player::NONE && turns.0 >= 3 {
                        game_end(new_board.gameover());
                    }
                    if let Board::Board(ref mut inner_board) = board.as_mut() {
                        *inner_board = new_board.unwrap(); // Modify the inner value
                    }
                    queue.0.push(
                        (Timer::new(Duration::from_millis(0), TimerMode::Once), Add_Queue_data {player, coordinates}));
                    println!("pushed new add to queue")
                },
                Err(e) => {
                    println!("{e}");
                }
            }




        println!("{}", board.render());
    } else {
        println!("Not a valid input. Please try again.");
    }
    1
}

fn game_end(player: Player) {
    match player {
        PLAYER2 => {
            println!("Game Over!");
            println!("Player 1 wins!");
        }
        PLAYER1 => {
            println!("Game Over!");
            println!("Player 2 wins!");
        },
        _ => return
    }
}