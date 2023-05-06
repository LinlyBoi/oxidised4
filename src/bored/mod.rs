use raylib::prelude::Color;

use crate::gamedata::{Board, Disk};

#[cfg(test)]
mod tests;
pub struct PlayState {
    pub circles: Vec<(i32, i32, Disk)>,
    pub bottom: Vec<i32>,
    pub player_turn: bool,
    pub board: Board,
}
impl Default for PlayState {
    fn default() -> Self {
        Self {
            circles: vec![],
            bottom: vec![],
            player_turn: false,
            board: Board::default(),
        }
    }
}
pub struct MenuState {
    difficulty: i32,
    p1: (Color, Disk),
    p2: (Color, Disk),
}
impl Default for MenuState {
    fn default() -> Self {
        Self {
            difficulty: 3,
            p1: (Color::RED, Disk::P1),
            p2: (Color::YELLOW, Disk::P2),
        }
    }
}
pub enum GameState {
    Play(PlayState),
    MainMenu(MenuState),
}
