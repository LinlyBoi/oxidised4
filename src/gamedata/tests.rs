use crate::gamedata::score_checkers::Direction;

use super::*;

// #[test]
// fn board_default() {
//     unimplemented!()
// }

#[test]
fn play() {
    let mut board = Board::default();
    assert!(board.play(Disk::BLU, 0));
    assert!(board.play(Disk::BLU, 0));
    assert!(board.play(Disk::BLU, 0));
    assert!(board.play(Disk::BLU, 0));
    assert!(board.play(Disk::BLU, 0));
    assert!(board.play(Disk::BLU, 0));
    assert!(board.play(Disk::BLU, 0));
    assert!(!board.play(Disk::BLU, 0));

    assert!(board.play(Disk::BLU, 1));
    assert!(board.play(Disk::BLU, 1));
    assert!(board.play(Disk::BLU, 1));
    assert!(board.play(Disk::BLU, 1));
    assert!(board.play(Disk::BLU, 1));
    assert!(board.play(Disk::BLU, 1));
    assert!(board.play(Disk::BLU, 1));
    assert!(!board.play(Disk::BLU, 1));

    assert!(board.play(Disk::BLU, 2));
    assert!(board.play(Disk::BLU, 2));
    assert!(board.play(Disk::BLU, 2));
    assert!(board.play(Disk::BLU, 2));
    assert!(board.play(Disk::BLU, 2));
    assert!(board.play(Disk::BLU, 2));
    assert!(board.play(Disk::BLU, 2));
    assert!(!board.play(Disk::BLU, 2));

    assert!(board.play(Disk::BLU, 3));
    assert!(board.play(Disk::BLU, 3));
    assert!(board.play(Disk::BLU, 3));
    assert!(board.play(Disk::BLU, 3));
    assert!(board.play(Disk::BLU, 3));
    assert!(board.play(Disk::BLU, 3));
    assert!(board.play(Disk::BLU, 3));
    assert!(!board.play(Disk::BLU, 3));
}
#[test]
fn scan_updown() {
    let mut board = Board::default();
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    assert_eq!(3, scan(&board.columns, &(4, 0), Direction::DOWN, 3));
    assert_eq!(3, scan(&board.columns, &(3, 0), Direction::DOWN, 3));
}
#[test]
fn scan_updown2() {
    let mut board = Board::default();
    board.play(Disk::BLU, 0);
    board.play(Disk::RED, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    assert_eq!(0, scan(&board.columns, &(0, 0), Direction::UP, 3));
}
#[test]
fn scan_forwardback() {
    let mut board = Board::default();
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 1);
    board.play(Disk::BLU, 2);
    board.play(Disk::BLU, 3);

    assert!(!matches!(Disk::RED, Disk::BLU));
    assert_eq!(1, scan(&board.columns, &(0, 0), Direction::RIGHT, 3));
    assert_eq!(1, scan(&board.columns, &(0, 3), Direction::LEFT, 3));
}
#[test]
fn scan_forwardback2() {
    let mut board = Board::default();
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 1);
    board.play(Disk::RED, 2);
    board.play(Disk::BLU, 3);
    assert_eq!(0, scan(&board.columns, &(0, 0), Direction::RIGHT, 3));
    assert_eq!(0, scan(&board.columns, &(0, 3), Direction::LEFT, 3));
}
#[test]
fn scan_diag1() {
    let mut board = Board::default();
    board.play(Disk::BLU, 0);
    board.play(Disk::RED, 1);
    board.play(Disk::BLU, 1);
    board.play(Disk::RED, 2);
    board.play(Disk::RED, 2);
    board.play(Disk::BLU, 2);
    board.play(Disk::RED, 3);
    board.play(Disk::RED, 3);
    board.play(Disk::RED, 3);
    board.play(Disk::BLU, 3);
    assert_eq!(1, scan(&board.columns, &(0, 0), Direction::UPRIGHT, 3));
    assert_eq!(1, scan(&board.columns, &(3, 3), Direction::DOWNLEFT, 3));
}
#[test]
fn scan_diag2() {
    let mut board = Board::default();
    board.play(Disk::BLU, 3);
    board.play(Disk::RED, 2);
    board.play(Disk::BLU, 2);
    board.play(Disk::RED, 1);
    board.play(Disk::RED, 1);
    board.play(Disk::BLU, 1);
    board.play(Disk::RED, 0);
    board.play(Disk::RED, 0);
    board.play(Disk::RED, 0);
    board.play(Disk::BLU, 0);
    dbg!(&board.columns.as_columns());
    assert_eq!(1, scan(&board.columns, &(0, 3), Direction::UPLEFT, 3));
    assert_eq!(1, scan(&board.columns, &(3, 0), Direction::DOWNRIGHT, 3));
}
#[test]
fn variant_eq_test() {
    assert!(score_checkers::variant_eq(&Disk::RED, &Disk::RED));
    assert!(matches!(Disk::RED, Disk::RED));
    assert!(!score_checkers::variant_eq(&Disk::BLU, &Disk::RED));
    assert!(!matches!(Disk::BLU, Disk::RED));
}
