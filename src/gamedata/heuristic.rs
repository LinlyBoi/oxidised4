use array2d::Array2D;

use super::{
    dec_both, dec_col, dec_inc, dec_row, inc_both, inc_col, inc_dec, inc_row,
    score_checkers::{get_legal_moves, variant_eq, Direction},
    Board, Disk,
};
//multipliers
const POT_STREAK: i32 = 4; //one streak is kind of poopy
const POT_STREAKS: i32 = 6;
const POT_WIN: i32 = 5; // should be nerfed if its just 1 potential win
const POT_WINS: i32 = 8;
const SCORE_DIFF: i32 = 6;
const MAX_WINS: i32 = 17;

pub fn get_score(board: &Board, disk: Disk) -> i32 {
    //this should be summing up a bunch of functions defined below this one
    let score: i32 = match disk {
        Disk::RED => board.red_score - board.blu_score,
        Disk::BLU => board.blu_score - board.red_score,
        Disk::EMPTY => panic!("Why would you ever"),
    };
    potential_streaks(&board.columns, &disk)
        + potential_wins(&board.columns, &disk)
        + score * SCORE_DIFF
}
pub fn potential_wins(board: &Array2D<Disk>, disk: &Disk) -> i32 {
    let pot_wins = get_dups(board, disk);
    dbg!(&pot_wins);
    let mut count: i32 = 0;
    for win in pot_wins {
        if win
            .iter()
            .filter(|&_disk| variant_eq(_disk, &Disk::EMPTY))
            .count()
            == 1
            && win.len() == 4
        {
            count += 1;
        }
    }
    match count {
        1 => POT_WIN,
        _ => POT_WINS * count,
    }
}
pub fn potential_streaks(board: &Array2D<Disk>, disk: &Disk) -> i32 {
    //This should grab potential streaks (Disk::EMPTY)
    // get all middle indexes
    let streaks = get_dups(board, disk);
    match streaks.iter().count() {
        1 => POT_STREAK,
        _ => POT_STREAKS * streaks.iter().count() as i32,
    }
}
fn get_dups(board: &Array2D<Disk>, player_disk: &Disk) -> Vec<Vec<Disk>> {
    let edge_col = board.num_columns() - 1;
    let mid_col = edge_col / 2;
    let mid_indices: Vec<(usize, usize)> = board
        .indices_row_major()
        .filter(|&index| variant_eq(board.get(index.0, index.1).expect(""), &Disk::EMPTY))
        .collect();
    let mut dups: Vec<Vec<Disk>> = vec![];
    for index in mid_indices {
        let moves = get_legal_moves(&index, (board.num_rows(), board.num_columns()));
        for direction in moves {
            let poopy = heur_scan(&board, &index, direction.clone(), 4, *player_disk);
            //dbg!(&index, &direction, &poopy);
            match poopy
                .iter()
                .filter(|&disk| variant_eq(disk, player_disk))
                .count()
            {
                3 | 2 => dups.push(poopy),
                _ => {}
            }
        }
    }
    dups
}

fn heur_scan(
    board: &Array2D<Disk>,
    index: &(usize, usize),
    direction: Direction,
    depth: i32,
    player_disk: Disk,
) -> Vec<Disk> {
    match board.get(index.0, index.1) {
        Some(_) => {}
        None => return vec![],
    };
    let mut current_index = *index;
    let mut in_a_row: Vec<Disk> = vec![];
    // dbg!("Starting new thing", &direction);
    for _num in 0..depth {
        match board.get(current_index.0, current_index.1) {
            Some(_disk) => {
                //            dbg!(_disk, current_disk, in_a_row);
                if variant_eq(&player_disk, _disk) || variant_eq(_disk, &Disk::EMPTY) {
                    // add in a row by 1
                    in_a_row.push(*_disk);
                    //               dbg!(current_index);
                    //go to next element
                    match direction {
                        Direction::DOWN => {
                            if current_index.0 == 0 {
                                break;
                            }
                            current_index = dec_row(&current_index, 1);
                        }
                        Direction::UP => {
                            if current_index.0 == board.num_rows() - 1 {
                                break;
                            }
                            current_index = inc_row(&current_index, 1);
                        }
                        Direction::LEFT => {
                            if current_index.1 == 0 {
                                break;
                            }
                            current_index = dec_col(&current_index, 1);
                        }
                        Direction::RIGHT => {
                            if current_index.1 == board.num_columns() - 1 {
                                break;
                            }
                            current_index = inc_col(&current_index, 1);
                        }
                        Direction::UPRIGHT => {
                            if current_index.0 == board.num_rows() - 1
                                || current_index.1 == board.num_columns() - 1
                            {
                                break;
                            }
                            current_index = inc_both(&current_index, 1);
                        }
                        Direction::UPLEFT => {
                            if current_index.0 == board.num_columns() - 1 || current_index.1 == 0 {
                                break;
                            }

                            current_index = inc_dec(&current_index, 1);
                        }
                        Direction::DOWNRIGHT => {
                            if current_index.0 == 0 || current_index.1 == board.num_columns() - 1 {
                                break;
                            }
                            current_index = dec_inc(&current_index, 1);
                        }
                        Direction::DOWNLEFT => {
                            if current_index.0 == 0 || current_index.1 == 0 {
                                break;
                            }
                            current_index = dec_both(&current_index, 1);
                        }
                    }
                } else {
                    break;
                }
            }

            None => break,
        }
    }
    in_a_row
}
