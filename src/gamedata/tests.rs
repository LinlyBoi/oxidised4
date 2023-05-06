use self::score_checkers::scan;
use crate::gamedata::score_checkers::Direction;

use super::*;

// #[test]
// fn board_default() {
//     unimplemented!()
// }

#[test]
fn play() {
    let mut board = Board::default();
    assert!(board.play(Disk::P2, 0));
    assert!(board.play(Disk::P2, 0));
    assert!(board.play(Disk::P2, 0));
    assert!(board.play(Disk::P2, 0));
    assert_eq!(1, board.p2_score);
    assert!(board.play(Disk::P2, 0));
    assert!(board.play(Disk::P2, 0));
    assert!(!board.play(Disk::P2, 0));

    assert!(board.play(Disk::P2, 1));
    assert!(board.play(Disk::P2, 1));
    assert!(board.play(Disk::P2, 1));
    assert!(board.play(Disk::P2, 1));
    assert!(board.play(Disk::P2, 1));
    assert!(board.play(Disk::P2, 1));
    assert!(!board.play(Disk::P2, 1));

    assert!(board.play(Disk::P2, 2));
    assert!(board.play(Disk::P2, 2));
    assert!(board.play(Disk::P2, 2));
    assert!(board.play(Disk::P2, 2));
    assert!(board.play(Disk::P2, 2));
    assert!(board.play(Disk::P2, 2));
    assert!(!board.play(Disk::P2, 2));

    assert!(board.play(Disk::P2, 3));
    assert!(board.play(Disk::P2, 3));
    assert!(board.play(Disk::P2, 3));
    assert!(board.play(Disk::P2, 3));
    assert!(board.play(Disk::P2, 3));
    assert!(board.play(Disk::P2, 3));
    assert!(!board.play(Disk::P2, 3));
}
#[test]
fn scan_updown() {
    let mut board = Board::default();
    board.play(Disk::P2, 0);
    board.play(Disk::P2, 0);
    board.play(Disk::P2, 0);
    board.play(Disk::P2, 0);
    board.play(Disk::P2, 0);
    board.play(Disk::P2, 0);
    board.play(Disk::P2, 0);
    assert_eq!(4, scan(&board.columns, &(4, 0), Direction::Down, 4));
    assert_eq!(4, scan(&board.columns, &(3, 0), Direction::Down, 4));
}
#[test]
fn scan_updown2() {
    let mut board = Board::default();
    board.play(Disk::P2, 0);
    board.play(Disk::P1, 0);
    board.play(Disk::P2, 0);
    board.play(Disk::P2, 0);
    assert_eq!(1, scan(&board.columns, &(0, 0), Direction::Up, 4));
}
#[test]
fn scan_forwardback() {
    let mut board = Board::default();
    board.play(Disk::P2, 0);
    board.play(Disk::P2, 1);
    board.play(Disk::P2, 2);
    board.play(Disk::P2, 3);

    assert_eq!(4, scan(&board.columns, &(0, 0), Direction::Right, 4));
    assert_eq!(4, scan(&board.columns, &(0, 3), Direction::Left, 4));
}
#[test]
fn scan_forwardback2() {
    let mut board = Board::default();
    board.play(Disk::P2, 0);
    board.play(Disk::P2, 1);
    board.play(Disk::P1, 2);
    board.play(Disk::P2, 3);
    assert_eq!(2, scan(&board.columns, &(0, 0), Direction::Right, 4));
    assert_eq!(1, scan(&board.columns, &(0, 3), Direction::Left, 4));
}
#[test]
fn scan_diag1() {
    let mut board = Board::default();
    board.play(Disk::P2, 0);
    board.play(Disk::P1, 1);
    board.play(Disk::P2, 1);
    board.play(Disk::P1, 2);
    board.play(Disk::P1, 2);
    board.play(Disk::P2, 2);
    board.play(Disk::P1, 3);
    board.play(Disk::P1, 3);
    board.play(Disk::P1, 3);
    board.play(Disk::P2, 3);
    assert_eq!(4, scan(&board.columns, &(0, 0), Direction::UpRight, 4));
    assert_eq!(4, scan(&board.columns, &(3, 3), Direction::DownLeft, 4));
}
#[test]
fn scan_diag2() {
    let mut board = Board::default();
    board.play(Disk::P2, 3);
    board.play(Disk::P1, 2);
    board.play(Disk::P2, 2);
    board.play(Disk::P1, 1);
    board.play(Disk::P1, 1);
    board.play(Disk::P2, 1);
    board.play(Disk::P1, 0);
    board.play(Disk::P1, 0);
    board.play(Disk::P1, 0);
    board.play(Disk::P2, 0);
    dbg!(&board.columns.as_columns());
    assert_eq!(4, scan(&board.columns, &(0, 3), Direction::UpLeft, 4));
    assert_eq!(4, scan(&board.columns, &(3, 0), Direction::DownRight, 4));
}
#[test]
fn variant_eq_test() {
    assert!(score_checkers::variant_eq(&Disk::P1, &Disk::P1));
    assert!(matches!(Disk::P1, Disk::P1));
    assert!(matches!(&Disk::P2, &Disk::P2));
    assert!(!score_checkers::variant_eq(&Disk::P2, &Disk::P1));
    assert!(!matches!(Disk::P2, Disk::P1));
}
#[test]
fn game_over_test() {
    let mut board = Board::default();
    assert!(!board.game_over());
    board.columns = Array2D::filled_with(Disk::P2, 7, 6);
    assert!(board.game_over());
    board.columns.set(0, 0, Disk::EMPTY).expect("balls");
    assert!(!board.game_over());
}
