use crate::gamedata::Board;

use super::Disk;

#[test]
fn board_default() {
    assert_eq!(7, Board::default().columns.len())
}
#[test]
fn play() {
    let mut board = Board::default();
    assert!(board.columns.get(0).expect("Nah").is_empty());
    board.play(Disk::BLUE, 0);
    assert_eq!(1, board.columns[0].len());
    board.play(Disk::RED, 0);
    assert_eq!(2, board.columns[0].len());
}
