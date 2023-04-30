use crate::gamedata::{
    score_checkers::{one_direction, Direction},
    Board,
};

use super::Disk;

// #[test]
// fn board_default() {
//     unimplemented!()
// }

#[test]
fn play() {
    let mut board = Board::default();
    assert!(board.play(Disk::BLUE, 0));
    assert!(board.play(Disk::BLUE, 0));
    assert!(board.play(Disk::BLUE, 0));
    assert!(board.play(Disk::BLUE, 0));
    assert!(board.play(Disk::BLUE, 0));
    assert!(board.play(Disk::BLUE, 0));
    assert!(board.play(Disk::BLUE, 0));
    assert!(!board.play(Disk::BLUE, 0));

    assert!(board.play(Disk::BLUE, 1));
    assert!(board.play(Disk::BLUE, 1));
    assert!(board.play(Disk::BLUE, 1));
    assert!(board.play(Disk::BLUE, 1));
    assert!(board.play(Disk::BLUE, 1));
    assert!(board.play(Disk::BLUE, 1));
    assert!(board.play(Disk::BLUE, 1));
    assert!(!board.play(Disk::BLUE, 1));

    assert!(board.play(Disk::BLUE, 2));
    assert!(board.play(Disk::BLUE, 2));
    assert!(board.play(Disk::BLUE, 2));
    assert!(board.play(Disk::BLUE, 2));
    assert!(board.play(Disk::BLUE, 2));
    assert!(board.play(Disk::BLUE, 2));
    assert!(board.play(Disk::BLUE, 2));
    assert!(!board.play(Disk::BLUE, 2));

    assert!(board.play(Disk::BLUE, 3));
    assert!(board.play(Disk::BLUE, 3));
    assert!(board.play(Disk::BLUE, 3));
    assert!(board.play(Disk::BLUE, 3));
    assert!(board.play(Disk::BLUE, 3));
    assert!(board.play(Disk::BLUE, 3));
    assert!(board.play(Disk::BLUE, 3));
    assert!(!board.play(Disk::BLUE, 3));
}
#[test]
fn one_direction_test() {
    let mut board_true = Board::default();
    let board_false = Board::default();
    board_true.play(Disk::BLUE, 0);
    board_true.play(Disk::BLUE, 0);
    board_true.play(Disk::BLUE, 0);
    board_true.play(Disk::BLUE, 0);
    assert_eq!(
        1,
        one_direction(&board_true.columns, &(3, 0), Direction::BACKWARD)
    );
    assert_eq!(
        0,
        one_direction(&board_false.columns, &(3, 0), Direction::BACKWARD)
    );
}
