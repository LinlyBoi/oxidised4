use array2d::Array2D;

use super::Disk;

pub fn one_direction(board: &Array2D<Disk>, index: &(usize, usize), direction: Direction) -> i32 {
    let current_disk: &Disk;
    match board.get(index.0, index.1) {
        Some(disk) => current_disk = disk,
        None => return 0,
    };
    let mut current_index = *index;
    let mut in_a_row = 0;
    loop {
        match board.get(current_index.0, current_index.1) {
            Some(_disk) => {
                if variant_eq(current_disk, _disk) && !matches!(_disk, Disk::EMPTY) {
                    // add in a row by 1
                    in_a_row += 1;
                    //go to next element
                    match direction {
                        Direction::DOWN => {
                            if current_index.0 == 0 {
                                break;
                            }
                            current_index.0 -= 1;
                        }
                        Direction::UP => {
                            if current_index.0 == board.num_columns() - 1 {
                                break;
                            }
                            current_index.0 += 1;
                        }
                        Direction::BACKWARD => {
                            if current_index.1 == 0 {
                                break;
                            }
                            current_index.1 -= 1;
                        }
                        Direction::FORWARD => {
                            if current_index.1 == board.num_rows() - 1 {
                                break;
                            }
                            current_index.1 += 1;
                        }
                        Direction::UPFORW => {
                            if current_index.0 == board.num_columns() - 1
                                || current_index.1 == board.num_rows() - 1
                            {
                                break;
                            }
                            current_index.1 += 1;
                            current_index.0 += 1;
                        }
                        Direction::UPBACK => {
                            if current_index.0 == board.num_columns() - 1 || current_index.1 == 0 {
                                break;
                            }
                            current_index.1 -= 1;
                            current_index.0 += 1;
                        }
                        Direction::DOWNFORW => {
                            if current_index.0 == 0 || current_index.1 == board.num_columns() - 1 {
                                break;
                            }
                            current_index.1 += 1;
                            current_index.0 -= 1;
                        }
                        Direction::DOWNBACK => {
                            if current_index.0 == 0 || current_index.1 == 0 {
                                break;
                            }
                            current_index.1 -= 1;
                            current_index.0 -= 1;
                        }
                    }
                } else {
                    break;
                }
            }

            None => break,
        }
    }
    if in_a_row == 4 {
        //score added
        return 1;
    } else {
        return 0;
    }
    //+-3
}
// board[(2,3)];

pub fn two_direction(board: &Array2D<Disk>, index: &(usize, usize)) -> i32 {
    let current_disk = board.get(index.0, index.1);
    unimplemented!()
    //+-1 -+2
}
pub enum Direction {
    UP,
    DOWN,
    FORWARD,
    BACKWARD,
    UPFORW,
    UPBACK,
    DOWNFORW,
    DOWNBACK,
    //TODO add more directions for diagonals
}
// serves nothing except do what matches!() should have done all along
fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
