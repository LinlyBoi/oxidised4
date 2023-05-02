use array2d::Array2D;

use super::{
    dec_both, dec_col, dec_inc, dec_row, inc_both, inc_col, inc_dec, inc_row,
    score_checkers::{variant_eq, Direction},
    Board, Disk,
};
//multipliers
const POT_STREAK: i32 = 4; //one streak is kind of poopy
const POT_STREAKS: i32 = 6;
const POT_WIN: i32 = 5; // should be nerfed if its just 1 potential win
const POT_WINS: i32 = 8;
const SCORE_DIFF: i32 = 6;
const MAX_WINS: i32 = 17;

pub fn get_score(board: &Board) -> i32 {
    //this should be summing up a bunch of functions defined below this one
    todo!()
}
pub fn potential_wins(board: &Array2D<Disk>) -> i32 {
    //3 of same kind and 4th EMPTY
    todo!()
}
pub fn potential_streaks(board: &Array2D<Disk>) -> i32 {
    //This should grab potential streaks (Disk::EMPTY)
    todo!()
}
fn heur_scan(
    board: &Array2D<Disk>,
    index: &(usize, usize),
    direction: Direction,
    depth: i32,
) -> Vec<Disk> {
    let current_disk: &Disk;
    match board.get(index.0, index.1) {
        Some(disk) => current_disk = disk,
        None => return vec![],
    };
    let mut current_index = *index;
    let mut in_a_row: Vec<Disk> = vec![];
    // dbg!("Starting new thing", &direction);
    for _num in 0..depth {
        match board.get(current_index.0, current_index.1) {
            Some(_disk) => {
                //            dbg!(_disk, current_disk, in_a_row);
                if variant_eq(current_disk, _disk) || variant_eq(_disk, &Disk::EMPTY) {
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
