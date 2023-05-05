use super::{flip_disk, heuristic::get_score, Board, Disk};
pub fn minimax_decision(board: &Board, disk: Disk, depth: &i32) -> Board {
    let (child, _) = maximise(board, &disk, depth);
    match child {
        Some(state) => state,
        None => Board::default(),
    }
}
fn maximise(board: &Board, disk: &Disk, depth: &i32) -> (Option<Board>, i32) {
    match board.game_over() || *depth == 0 {
        true => return (None, get_score(board, flip_disk(*disk))),
        false => {
            let (mut max_child, mut max_utility): (Option<Board>, i32) = (None, i32::MIN);
            for child in board.get_children(*disk) {
                let (_, utility) = minimise(&child, &flip_disk(*disk), &(depth - 1));
                if utility > max_utility {
                    (max_child, max_utility) = (Some(child), utility)
                }
            }
            (max_child, max_utility)
        }
    }
}
fn minimise(board: &Board, disk: &Disk, depth: &i32) -> (Option<Board>, i32) {
    match board.game_over() || *depth == 0 {
        true => return (None, get_score(board, flip_disk(*disk))),
        false => {
            let (mut min_child, mut min_utility): (Option<Board>, i32) = (None, i32::MIN);
            for child in board.get_children(*disk) {
                let (_, utility) = maximise(&child, &flip_disk(*disk), &(depth - 1));
                if utility > min_utility {
                    (min_child, min_utility) = (Some(child), utility)
                }
            }
            (min_child, min_utility)
        }
    }
}
#[test]
fn minimax_test() {
    let mut board = Board::default();
    dbg!(minimax_decision(&board, Disk::BLU, &5).columns.as_rows());
    assert!(false);
}
