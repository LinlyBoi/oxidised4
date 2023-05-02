use array2d::Array2D;

use crate::gamedata::{dec_both, dec_inc, dec_row, inc_both, inc_dec, inc_row};

use super::{dec_col, inc_col, Disk};

pub fn scan(
    board: &Array2D<Disk>,
    index: &(usize, usize),
    direction: Direction,
    depth: i32,
) -> i32 {
    let current_disk: &Disk;
    match board.get(index.0, index.1) {
        Some(disk) => current_disk = disk,
        None => return 0,
    };
    let mut current_index = *index;
    let mut in_a_row = 0;
    for _num in 0..depth {
        match board.get(current_index.0, current_index.1) {
            Some(_disk) => {
                dbg!(_disk, current_disk, in_a_row);
                if variant_eq(current_disk, _disk) && !variant_eq(_disk, &Disk::EMPTY) {
                    // add in a row by 1
                    in_a_row += 1;
                    dbg!(current_index);
                    //go to next element
                    match direction {
                        Direction::DOWN => {
                            if current_index.0 == 0 {
                                break;
                            }
                            current_index = dec_row(&current_index, 1);
                            //current_index.0 -= 1;
                        }
                        Direction::UP => {
                            if current_index.0 == board.num_rows() - 1 {
                                break;
                            }
                            current_index = inc_row(&current_index, 1);
                            //    current_index.0 += 1;
                        }
                        Direction::LEFT => {
                            if current_index.1 == 0 {
                                break;
                            }
                            current_index = dec_col(&current_index, 1);
                            //    current_index.1 -= 1;
                        }
                        Direction::RIGHT => {
                            if current_index.1 == board.num_columns() - 1 {
                                break;
                            }
                            current_index = inc_col(&current_index, 1);
                            //    current_index.1 += 1;
                        }
                        Direction::UPRIGHT => {
                            if current_index.0 == board.num_rows() - 1
                                || current_index.1 == board.num_columns() - 1
                            {
                                break;
                            }
                            current_index = inc_both(&current_index, 1);
                            //    current_index.1 += 1;
                            //    current_index.0 += 1;
                        }
                        Direction::UPLEFT => {
                            if current_index.0 == board.num_columns() - 1 || current_index.1 == 0 {
                                break;
                            }

                            //    current_index.1 -= 1;
                            //    current_index.0 += 1;
                            current_index = inc_dec(&current_index, 1);
                        }
                        Direction::DOWNRIGHT => {
                            if current_index.0 == 0 || current_index.1 == board.num_columns() - 1 {
                                break;
                            }
                            current_index = dec_inc(&current_index, 1);
                            //    current_index.1 += 1;
                            //    current_index.0 -= 1;
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
    //    //+-3
}
// board[(2,3)];
pub fn get_legal_moves(
    (row, col): &(usize, usize),
    direction: Direction,
    (nrow, ncol): (usize, usize),
) -> Vec<Direction> {
    let max_col = nrow - 1;
    let max_row = ncol - 1;
    let mut moves: Vec<Direction> = vec![];
    match *row {
        0 => moves.push(Direction::UP),
        max_row => moves.push(Direction::DOWN),
        _ => {
            moves.push(Direction::UP);
            moves.push(Direction::DOWN);
        }
    };
    match *col {
        0 => moves.push(Direction::RIGHT),
        max_row => moves.push(Direction::LEFT),
        _ => {
            moves.push(Direction::LEFT);
            moves.push(Direction::RIGHT)
        }
    };
    moves
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UPLEFT,
    UPRIGHT,
    DOWNLEFT,
    DOWNRIGHT,
}
// serves nothing except do what matches!() should have done all along
// matches works too I'm just dumb
pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
