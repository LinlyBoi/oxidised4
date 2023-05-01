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
fn one_direction_updown() {
    let mut board = Board::default();
    board.play(Disk::BLUE, 0);
    board.play(Disk::BLUE, 0);
    board.play(Disk::BLUE, 0);
    board.play(Disk::BLUE, 0);
    assert_eq!(1, one_direction(&board.columns, &(3, 0), Direction::DOWN));
}
#[test]
fn one_direction_updown2() {
    let mut board = Board::default();
    board.play(Disk::BLUE, 0);
    board.play(Disk::RED, 0);
    board.play(Disk::BLUE, 0);
    board.play(Disk::BLUE, 0);
    assert_eq!(0, one_direction(&board.columns, &(3, 0), Direction::DOWN));
}
#[test]
fn one_direction_forwardback() {
    let mut board = Board::default();
    board.play(Disk::BLUE, 0);
    board.play(Disk::BLUE, 1);
    board.play(Disk::BLUE, 2);
    board.play(Disk::BLUE, 3);

    assert!(!matches!(Disk::RED, Disk::BLUE));
    assert_eq!(
        1,
        one_direction(&board.columns, &(0, 0), Direction::FORWARD)
    );
    assert_eq!(
        1,
        one_direction(&board.columns, &(0, 3), Direction::BACKWARD)
    );
}
#[test]
fn one_direction_forwardback2() {
    let mut board = Board::default();
    board.play(Disk::BLUE, 0);
    board.play(Disk::BLUE, 1);
    board.play(Disk::BLUE, 3);
    board.play(Disk::RED, 2);
    assert_eq!(
        0,
        one_direction(&board.columns, &(0, 0), Direction::FORWARD)
    );
    assert_eq!(
        0,
        one_direction(&board.columns, &(0, 3), Direction::BACKWARD)
    );
}
