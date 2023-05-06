use crate::gamedata::{Board, Disk};

#[cfg(test)]
mod tests;
pub struct GameState {
    circles: Vec<(i32, i32, Disk)>,
    empty: Vec<i32>,
    player_turn: bool,
    board: Board,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            circles: vec![],
            empty: vec![],
            player_turn: false,
            board: Board::default(),
        }
    }
}
