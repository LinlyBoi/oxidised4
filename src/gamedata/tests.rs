use crate::gamedata::{dec_col, score_checkers::Direction, Board};

use super::{get_indices, inc_col, score_checkers::two_direction, Disk};

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
fn one_direction_updown() {
    let mut board = Board::default();
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    // assert_eq!(1, one_direction(&board.columns, &(4, 0), Direction::DOWN));
    // assert_eq!(1, one_direction(&board.columns, &(3, 0), Direction::DOWN));
}
#[test]
fn one_direction_updown2() {
    let mut board = Board::default();
    board.play(Disk::BLU, 0);
    board.play(Disk::RED, 0);
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 0);
    //  assert_eq!(0, one_direction(&board.columns, &(3, 0), Direction::DOWN));
}
#[test]
fn one_direction_forwardback() {
    let mut board = Board::default();
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 1);
    board.play(Disk::BLU, 2);
    board.play(Disk::BLU, 3);

    //  assert!(!matches!(Disk::RED, Disk::BLU));
    //  assert_eq!(
    //      1,
    //      one_direction(&board.columns, &(0, 0), Direction::FORWARD)
    //  );
    //  assert_eq!(
    //      1,
    //      one_direction(&board.columns, &(0, 3), Direction::BACKWARD)
    //  );
}
#[test]
fn one_direction_forwardback2() {
    let mut board = Board::default();
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 1);
    board.play(Disk::RED, 2);
    board.play(Disk::BLU, 3);
    //  assert_eq!(
    //      0,
    //      one_direction(&board.columns, &(0, 0), Direction::FORWARD)
    //  );
    //  assert_eq!(
    //      0,
    //      one_direction(&board.columns, &(0, 3), Direction::BACKWARD)
    //  );
}
#[test]
fn one_direction_diag1() {
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
    //   assert_eq!(1, one_direction(&board.columns, &(0, 0), Direction::UPFORW));
    //   assert_eq!(
    //       1,
    //       one_direction(&board.columns, &(3, 3), Direction::DOWNBACK)
    //   );
}
#[test]
fn one_direction_diag2() {
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
    //    assert_eq!(1, one_direction(&board.columns, &(0, 3), Direction::UPBACK));
    //    assert_eq!(
    //        1,
    //        one_direction(&board.columns, &(3, 0), Direction::DOWNFORW)
    //    );
}
#[test]
fn get_indices_test() {
    let values: Vec<usize> = vec![1, 2];
    let indices = get_indices(&(2, 1), inc_col, values);
    assert_eq!(vec![(2, 2), (2, 3)], indices);
    let indices = get_indices(&(2, 1), dec_col, vec![1]);
    assert_eq!(vec![(2, 0)], indices);
}

#[test]
fn two_direction_test() {
    let mut board = Board::default();
    board.play(Disk::BLU, 0);
    board.play(Disk::BLU, 1);
    board.play(Disk::BLU, 2);
    board.play(Disk::BLU, 3);
    board.play(Disk::BLU, 4);
    board.play(Disk::BLU, 5);
    let added = two_direction(&board.columns, &(0, 3), Direction::HORIZONTAL);
    assert_eq!(1, added)
}
