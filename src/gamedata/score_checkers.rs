use array2d::Array2D;

use crate::gamedata::{dec_both, dec_inc, dec_row, inc_both, inc_dec, inc_row};

use super::{dec_col, inc_col, Disk};

pub fn scan(
    board: &Array2D<Disk>,
    index: &(usize, usize),
    direction: Direction,
    depth: i32,
    player_disk: Disk,
) -> i32 {
    match board.get(index.0, index.1) {
        Some(_) => {}
        None => return 0,
    };
    let mut current_index = *index;
    let mut in_a_row: Vec<Disk> = vec![];
    // dbg!("Starting new thing", &direction);
    for _num in 0..depth {
        match board.get(current_index.0, current_index.1) {
            Some(_disk) => {
                //            dbg!(_disk, current_disk, in_a_row);
                if variant_eq(&player_disk, _disk) && !variant_eq(_disk, &Disk::EMPTY) {
                    // add in a row by 1
                    in_a_row.push(*_disk);
                    //               dbg!(current_index);
                    //go to next element
                    match direction {
                        Direction::Down => {
                            if current_index.0 == 0 {
                                break;
                            }
                            current_index = dec_row(&current_index, 1);
                        }
                        Direction::Up => {
                            if current_index.0 == board.num_rows() - 1 {
                                break;
                            }
                            current_index = inc_row(&current_index, 1);
                        }
                        Direction::Left => {
                            if current_index.1 == 0 {
                                break;
                            }
                            current_index = dec_col(&current_index, 1);
                        }
                        Direction::Right => {
                            if current_index.1 == board.num_columns() - 1 {
                                break;
                            }
                            current_index = inc_col(&current_index, 1);
                        }
                        Direction::UpRight => {
                            if current_index.0 == board.num_rows() - 1
                                || current_index.1 == board.num_columns() - 1
                            {
                                break;
                            }
                            current_index = inc_both(&current_index, 1);
                        }
                        Direction::UpLeft => {
                            if current_index.0 == board.num_columns() - 1 || current_index.1 == 0 {
                                break;
                            }

                            current_index = inc_dec(&current_index, 1);
                        }
                        Direction::DownRight => {
                            if current_index.0 == 0 || current_index.1 == board.num_columns() - 1 {
                                break;
                            }
                            current_index = dec_inc(&current_index, 1);
                        }
                        Direction::DownLeft => {
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
    in_a_row.len() as i32
}
// board[(2,3)];
pub fn get_legal_moves(
    (row, col): &(usize, usize),
    (nrow, ncol): (usize, usize),
) -> Vec<Direction> {
    let _max_col = ncol - 1;
    let _max_row = nrow - 1;
    let mut moves: Vec<Direction> = vec![];
    match *row {
        0 => moves.push(Direction::Up),
        _max_rows => moves.push(Direction::Down),
        _ => {
            moves.push(Direction::Up);
            moves.push(Direction::Down);
        }
    };
    match *col {
        0 => moves.push(Direction::Right),
        _max_col => moves.push(Direction::Left),
        _ => {
            moves.push(Direction::Left);
            moves.push(Direction::Right)
        }
    };
    if moves.contains(&Direction::Up) && moves.contains(&Direction::Left) {
        moves.push(Direction::UpLeft);
    }
    if moves.contains(&Direction::Up) && moves.contains(&Direction::Right) {
        moves.push(Direction::UpRight);
    }
    if moves.contains(&Direction::Down) && moves.contains(&Direction::Left) {
        moves.push(Direction::DownLeft);
    }
    if moves.contains(&Direction::Down) && moves.contains(&Direction::Right) {
        moves.push(Direction::DownRight);
    }
    moves
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
// serves nothing except do what matches!() should have done all along
// matches works too I'm just dumb
pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
pub fn flip_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::UpLeft => Direction::DownRight,
        Direction::UpRight => Direction::DownLeft,
        Direction::DownLeft => Direction::UpRight,
        Direction::DownRight => Direction::UpLeft,
    }
}
