use crate::gamedata::{Board, Disk};

#[cfg(test)]
mod tests;
pub struct GameState {
    pub circles: Vec<(i32, i32, Disk)>,
    pub bottom: Vec<i32>,
    pub player_turn: bool,
    pub board: Board,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            circles: vec![],
            bottom: vec![],
            player_turn: false,
            board: Board::default(),
        }
    }
}
