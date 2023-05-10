use std::fmt::{self, Display};

use raylib::{
    prelude::{Color, MouseButton},
    RaylibHandle,
};

use crate::gamedata::{Board, Disk};
pub const STARTY: i32 = 9;
pub const STARTX: i32 = 7;
const WX: i32 = 14;
const WY: i32 = 14;
const CIRCLEWIDTH: i32 = 56;

#[cfg(test)]
mod tests;
#[derive(Clone)]
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
            bottom: vec![6, 6, 6, 6, 6, 6, 6],
            player_turn: true,
            board: Board::default(),
        }
    }
}
impl PlayState {
    pub fn play_human(&mut self, column: i32) {
        self.board.play(Disk::P1, column as usize);

        self.bottom[column as usize] -= 1;
        let (x, y) = get_circle_coords(column, self.bottom[column as usize]);
        self.circles.push((x, y, Disk::P1));
        self.player_turn = false;
    }
    pub fn play_cpu(&mut self, diff: &i32, cook: fn(&Board, Disk, &i32) -> Board) {
        self.board.play(
            Disk::P2,
            cook(&self.board, Disk::P2, diff).last_move as usize,
        );
        let column: i32 = self.board.last_move;
        self.bottom[column as usize] -= 1;
        let (x, y) = get_circle_coords(column, self.bottom[column as usize]);
        self.circles.push((x, y, Disk::P2));
        self.player_turn = false;
        self.player_turn = true;
    }
}
#[derive(Clone)]
pub struct MenuState {
    pub difficulty: i32,
    p1: (Color, Disk),
    p2: (Color, Disk),
    pub selection: Selection,
    pub strategy: Strategy,
}
#[derive(Clone, Debug)]
pub enum Strategy {
    MiniMax,
    AlphaBeta,
}
impl Default for MenuState {
    fn default() -> Self {
        Self {
            difficulty: 3,
            p1: (Color::RED, Disk::P1),
            p2: (Color::YELLOW, Disk::P2),
            selection: Selection::Difficulty,
            strategy: Strategy::MiniMax,
        }
    }
}
impl MenuState {
    pub fn init(&mut self, rl: &RaylibHandle) -> bool {
        dbg!(&self.selection);
        match self.selection {
            Selection::Difficulty => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    self.difficulty = 3;
                    self.selection = Selection::Cooking;
                }
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
                    self.difficulty = 5;
                    self.selection = Selection::Cooking;
                }
            }
            Selection::Cooking => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    self.strategy = Strategy::MiniMax;
                    self.selection = Selection::Done;
                }
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
                    self.strategy = Strategy::AlphaBeta;
                    self.selection = Selection::Done;
                }
            }
            Selection::Done => return true,
        }
        false
    }
}
#[derive(Clone)]
pub enum GameState {
    Play(PlayState),
    MainMenu(MenuState),
}
#[derive(Clone, Debug)]
pub enum Selection {
    Difficulty,
    Cooking,
    Done,
}
impl Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: String;
        match self {
            Self::MiniMax => name = String::from("MiniMax"),
            Self::AlphaBeta => name = String::from("AlphaBeta"),
        };
        write!(f, "{}", name)
    }
}

fn get_circle_coords(x: i32, y: i32) -> (i32, i32) {
    let mut returned: (i32, i32) = (0, 0);
    returned.0 = STARTY + (CIRCLEWIDTH * x) + (WX * x);
    returned.1 = STARTX + (CIRCLEWIDTH * y) + (WY * y);
    returned
}
