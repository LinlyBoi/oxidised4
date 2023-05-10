use std::time::{self, Instant};

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
        true => (None, get_score(board, *disk)),
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
        true => (None, get_score(board, flip_disk(*disk))),
        false => {
            let (mut min_child, mut min_utility): (Option<Board>, i32) = (None, i32::MAX);
            for child in board.get_children(*disk) {
                let (_, utility) = maximise(&child, &flip_disk(*disk), &(depth - 1));
                if utility < min_utility {
                    (min_child, min_utility) = (Some(child), utility)
                }
            }
            (min_child, min_utility)
        }
    }
}

pub fn minimax_decision_pruning(board: &Board, disk: Disk, depth: &i32) -> Board {
    let (child, _) = maximise_pruning(board, &disk, depth, i32::MIN, i32::MAX);
    match child {
        Some(state) => state,
        None => Board::default(),
    }
}
fn maximise_pruning(
    board: &Board,
    disk: &Disk,
    depth: &i32,
    mut alpha: i32,
    beta: i32,
) -> (Option<Board>, i32) {
    match board.game_over() || *depth == 0 {
        true => (None, get_score(board, *disk)),
        false => {
            let (mut max_child, mut max_utility): (Option<Board>, i32) = (None, i32::MIN);
            for child in board.get_children(*disk) {
                let (_, utility) =
                    minimise_pruning(&child, &flip_disk(*disk), &(depth - 1), alpha, beta);
                if beta <= alpha {
                    break;
                }
                if utility > alpha {
                    alpha = utility;
                }

                if utility > max_utility {
                    (max_child, max_utility) = (Some(child), utility)
                }
            }
            (max_child, max_utility)
        }
    }
}
fn minimise_pruning(
    board: &Board,
    disk: &Disk,
    depth: &i32,
    alpha: i32,
    mut beta: i32,
) -> (Option<Board>, i32) {
    match board.game_over() || *depth == 0 {
        true => (None, get_score(board, flip_disk(*disk))),
        false => {
            let (mut min_child, mut min_utility): (Option<Board>, i32) = (None, i32::MAX);
            for child in board.get_children(*disk) {
                let (_, utility) =
                    maximise_pruning(&child, &flip_disk(*disk), &(depth - 1), alpha, beta);
                if beta <= alpha {
                    break;
                }
                if utility < beta {
                    beta = utility;
                }
                if utility < min_utility {
                    (min_child, min_utility) = (Some(child), utility)
                }
            }
            (min_child, min_utility)
        }
    }
}
#[test]
fn minimax_test() {
    let time = Instant::now();
    let mut board = Board::default();
    let mut disk = Disk::P2;
    let _depth = 5;
    let _turn1 = board.play(disk, minimax_decision(&board, disk, &5).last_move as usize);
    dbg!(&board.columns);
    disk = flip_disk(disk);
    let _turn2 = board.play(disk, minimax_decision(&board, disk, &5).last_move as usize);
    disk = flip_disk(disk);
    let _turn3 = board.play(disk, minimax_decision(&board, disk, &5).last_move as usize);
    disk = flip_disk(disk);
    let _turn4 = board.play(disk, minimax_decision(&board, disk, &5).last_move as usize);
    disk = flip_disk(disk);
    let _turn5 = board.play(disk, minimax_decision(&board, disk, &5).last_move as usize);
    for column in board.columns.as_rows() {
        column
            .iter()
            .map(|x| {
                print!("{:#?},", x);
                x
            })
            .count();
        println!();
    }
    dbg!(time.elapsed().as_millis());
    assert!(false);
}
#[test]
fn minimax_pruning_test() {
    let time = Instant::now();
    let mut board = Board::default();
    let mut disk = Disk::P2;
    let _depth = 5;
    let _turn1 = board.play(
        disk,
        minimax_decision_pruning(&board, disk, &5).last_move as usize,
    );
    dbg!(&board.columns);
    disk = flip_disk(disk);
    let _turn2 = board.play(
        disk,
        minimax_decision_pruning(&board, disk, &5).last_move as usize,
    );
    disk = flip_disk(disk);
    let _turn3 = board.play(
        disk,
        minimax_decision_pruning(&board, disk, &5).last_move as usize,
    );
    disk = flip_disk(disk);
    let _turn4 = board.play(
        disk,
        minimax_decision_pruning(&board, disk, &5).last_move as usize,
    );
    disk = flip_disk(disk);
    let _turn5 = board.play(
        disk,
        minimax_decision_pruning(&board, disk, &5).last_move as usize,
    );
    for column in board.columns.as_rows() {
        column
            .iter()
            .map(|x| {
                print!("{:#?},", x);
                x
            })
            .count();
        println!();
    }
    dbg!(time.elapsed().as_millis());
    assert!(false);
}
