use crate::{UpdateQueue, BOARD_SIZE};
use core::cmp::Ord;
use std::cell::RefMut;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::time::Duration;
use bevy::prelude::*;
use crate::button::update_button_colour;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    PLAYER1,
    PLAYER2,
    PLAYER3,
    NONE
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize
}
impl Display for Coordinates {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "X: {} Y: {})", self.x, self.y)
    }
}
impl Coordinates {
    pub fn new(x: usize, y: usize) -> Option<Self> {
        if x < BOARD_SIZE[0] &&
            y < BOARD_SIZE[1]
        {
            return Some(
                Self {
                    x,
                    y
                }
            )
        }
        None
    }
}

#[derive(Debug)]
pub struct Add_Queue_data {
    pub player:Player,
    pub coordinates: Coordinates
}
#[derive(Clone, Copy, Debug, Resource)]
pub struct Tile {
    pub val: u8,
    pos: Coordinates,
    pub owner: Player,
    pub changed: bool
}
impl Default for Tile {
    fn default() -> Self {
        return Self {
            val: 0,
            pos: Coordinates::new(0,0).unwrap(),
            owner: Player::NONE,
            changed: false,
        }
    }
}
impl Tile {
    pub fn add(&mut self, player:Player) -> bool {
        // Adds one to self. Returns true if this would cause an explosion
        self.val+=1;
        self.owner = player;
        self.changed = true;
        if self.val > 3 {
            self.val = 0;
            self.owner = Player::NONE;
            return true
        }
        false
    }
}

#[derive(Clone, Copy, Debug, Resource)]
pub enum Board {Board([[Tile;BOARD_SIZE[0]];BOARD_SIZE[1]])}

impl Default for Board {
    fn default() -> Self {Self::Board([[Tile::default(); BOARD_SIZE[0]];BOARD_SIZE[1]])}
}

impl Board {
    pub fn unwrap(self) -> [[Tile;BOARD_SIZE[0]];BOARD_SIZE[1]] {
        match self {
            Board::Board(board) => board,
        }}
    pub fn init(&mut self) -> () {
        let mut board_iterable = self.unwrap();
        for mut y in 1..BOARD_SIZE[1] { // -1 because of different indexing measures
            for mut x in 1..BOARD_SIZE[0] - 1 {
                board_iterable[y][x].pos = Coordinates::new(x, y).expect(format!("Board generation left bounds. This should not have happened. Please message Maddie if you see this.\nCoordinates: X: {} | Y: {}", x, y).as_str());
            }
        }
        *self = Self::Board(board_iterable);
    }
    pub fn render(self) -> String {
        let mut board_buffer = String::new();
        for y in 0..BOARD_SIZE[1] {
            for x in 0..BOARD_SIZE[0] {
                board_buffer = format!("{board_buffer}{}{} {}",
                                       match self.unwrap()[y][x].owner{
                                           Player::PLAYER1 => "\x1b[0;31m", // Red
                                           Player::PLAYER2 => "\x1b[0;34m", // Blue
                                           _               => "\x1b[0m"  // White
                                       },
                                       self.unwrap()[y][x].val,
                                       "\x1b[0m");
            }
            board_buffer += "\n";
        }
        board_buffer

    }
    pub fn add(self, player:Player, coordinates: Coordinates, depth:u8, is_first_turn:bool, commands: &mut Commands, queue: &mut ResMut<UpdateQueue>) -> Result<Self, &'static str> {
        let mut board = self.unwrap();
        let mut is_exploded = false;
        if depth == 0 && (
            (board[coordinates.y][coordinates.x].owner != player && !is_first_turn) ||
                (board[coordinates.y][coordinates.x].owner != player &&
                    board[coordinates.y][coordinates.x].owner != Player::NONE))
        {return Err("Not a valid move. Please select a square on which you have already played.")}
        if coordinates.x  < 99999 && coordinates.y  < 99999 {is_exploded = board[coordinates.y][coordinates.x].add(player)}
        if  is_exploded {
            let left:Option<Coordinates> = Coordinates::new((coordinates.x as i32 -1) as usize, coordinates.y);
            let right:Option<Coordinates> = Coordinates::new((coordinates.x as i32 +1) as usize, coordinates.y);
            let up:Option<Coordinates> = Coordinates::new(coordinates.x, (coordinates.y as i32 -1) as usize);
            let down:Option<Coordinates> = Coordinates::new(coordinates.x, (coordinates.y as i32 +1) as usize);
            println!("left: {:?}", left);
            println!("right: {:?}", right);
            println!("up: {:?}", up);
            println!("down: {:?}", down);
            if left.is_some()  {
                board = Board::Board(board).add(player, left.unwrap(), depth+1, false, commands, queue)?.unwrap();
                queue.0.push(
                    (Timer::new(Duration::from_millis(500*depth as u64), TimerMode::Once), Add_Queue_data {player, coordinates:left.unwrap()}));
                println!("pushed new add to queue")}
            if right.is_some() {
                board = Board::Board(board).add(player, right.unwrap(),depth+1, false, commands, queue)?.unwrap();
                queue.0.push(
                    (Timer::new(Duration::from_millis(500*depth as u64), TimerMode::Once), Add_Queue_data {player, coordinates:right.unwrap()}));
                println!("pushed new add to queue")}
            if up.is_some()    {
                board = Board::Board(board).add(player, up.unwrap(),   depth+1, false, commands, queue)?.unwrap();
                queue.0.push(
                    (Timer::new(Duration::from_millis(500*depth as u64), TimerMode::Once), Add_Queue_data {player, coordinates:up.unwrap()}));
                println!("pushed new add to queue")}
            if down.is_some()  {
                board = Board::Board(board).add(player, down.unwrap(), depth+1, false, commands, queue)?.unwrap();
                queue.0.push(
                    (Timer::new(Duration::from_millis(500*depth as u64), TimerMode::Once), Add_Queue_data {player, coordinates:down.unwrap()}));
                println!("pushed new add to queue")}
        }

        Ok(Board::Board(board))
    }
    
    // Returns the losing player
    pub fn gameover(self) -> Player {
        let board_iterable = self.unwrap();
        let mut p2_loss:bool = true;
        let mut p1_loss:bool = true;
        for y in 0..BOARD_SIZE[1] {
            for x in 0..BOARD_SIZE[0] {
                if board_iterable[y][x].owner == Player::PLAYER1 {p1_loss = false}
                if board_iterable[y][x].owner == Player::PLAYER2 {p2_loss = false}
            }
        }
        if p1_loss {return Player::PLAYER1};
        if p2_loss {return Player::PLAYER2};
        return Player::NONE;
    }
}