use array2d::Array2D;

use super::{dec_col, get_indices, inc_col, Disk};

pub fn one_direction(board: &Array2D<Disk>, index: &(usize, usize), direction: Direction) -> i32 {
    //    let current_disk: &Disk;
    //    match board.get(index.0, index.1) {
    //        Some(disk) => current_disk = disk,
    //        None => return 0,
    //    };
    //    let mut current_index = *index;
    //    let mut in_a_row = 0;
    //    for _num in 0..4 {
    //        match board.get(current_index.0, current_index.1) {
    //            Some(_disk) => {
    //                if variant_eq(current_disk, _disk) && !matches!(_disk, Disk::EMPTY) {
    //                    // add in a row by 1
    //                    in_a_row += 1;
    //                    //go to next element
    //                    match direction {
    //                        Direction::DOWN => {
    //                            if current_index.0 == 0 {
    //                                break;
    //                            }
    //                            current_index.0 -= 1;
    //                        }
    //                        Direction::UP => {
    //                            if current_index.0 == board.num_columns() - 1 {
    //                                break;
    //                            }
    //                            current_index.0 += 1;
    //                        }
    //                        Direction::BACKWARD => {
    //                            if current_index.1 == 0 {
    //                                break;
    //                            }
    //                            current_index.1 -= 1;
    //                        }
    //                        Direction::FORWARD => {
    //                            if current_index.1 == board.num_rows() - 1 {
    //                                break;
    //                            }
    //                            current_index.1 += 1;
    //                        }
    //                        Direction::UPFORW => {
    //                            if current_index.0 == board.num_columns() - 1
    //                                || current_index.1 == board.num_rows() - 1
    //                            {
    //                                break;
    //                            }
    //                            current_index.1 += 1;
    //                            current_index.0 += 1;
    //                        }
    //                        Direction::UPBACK => {
    //                            if current_index.0 == board.num_columns() - 1 || current_index.1 == 0 {
    //                                break;
    //                            }
    //                            current_index.1 -= 1;
    //                            current_index.0 += 1;
    //                        }
    //                        Direction::DOWNFORW => {
    //                            if current_index.0 == 0 || current_index.1 == board.num_columns() - 1 {
    //                                break;
    //                            }
    //                            current_index.1 += 1;
    //                            current_index.0 -= 1;
    //                        }
    //                        Direction::DOWNBACK => {
    //                            if current_index.0 == 0 || current_index.1 == 0 {
    //                                break;
    //                            }
    //                            current_index.1 -= 1;
    //                            current_index.0 -= 1;
    //                        }
    //                    }
    //                } else {
    //                    break;
    //                }
    //            }
    //
    //            None => break,
    //        }
    //    }
    //    if in_a_row == 4 {
    //        //score added
    //        return 1;
    //    } else {
    //        return 0;
    //    }
    //    //+-3
    todo!()
}
// board[(2,3)];

pub fn two_direction(board: &Array2D<Disk>, index: &(usize, usize), direction: Direction) -> i32 {
    let mut added_score = 0;
    let current_disk: &Disk;
    match board.get(index.0, index.1) {
        Some(disk) => current_disk = disk,
        None => return 0,
    };
    match direction {
        Direction::HORIZONTAL => {
            //get values to increase/decrease by
            let two = vec![1, 2];
            let one = vec![1];
            //get surrounding indices
            let mut indices: Vec<(usize, usize)> = vec![];
            indices.append(&mut get_indices(index, inc_col, two));
            indices.append(&mut get_indices(index, dec_col, one));
            dbg!(indices.clone());
            let mut neighbours: Vec<Disk> = vec![];
            //get neighbours
            for index in indices {
                match board.get(index.0, index.1) {
                    Some(disk) => neighbours.push(*disk),
                    None => break,
                }
            }
            let in_a_row = neighbours
                .iter()
                .filter(|&a| variant_eq(a, current_disk))
                .count();
            if in_a_row == 3 {
                added_score += 1;
            }

            added_score
        }
        Direction::VERTICAL => todo!(),
        Direction::DIAGONAL => todo!(),
    }
}

pub enum Direction {
    HORIZONTAL,
    VERTICAL,
    DIAGONAL,
}
// serves nothing except do what matches!() should have done all along
fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
