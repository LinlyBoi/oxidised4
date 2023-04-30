use array2d::Array2D;

use super::Disk;

pub fn one_direction(board: &Array2D<Disk>, index: &(usize, usize), direction: Direction) -> i32 {
    let current_disk = board.get(index.0, index.1);
    let mut current_index = *index;
    let mut in_a_row = 0;
    loop {
        dbg!(in_a_row, current_index);
        match board.get(current_index.0, current_index.1) {
            Some(_disk) => {
                if matches!(current_disk, _disk) && !matches!(_disk, Disk::EMPTY) {
                    // add in a row by 1
                    in_a_row += 1;
                    //go to next element
                    match direction {
                        Direction::BACKWARD => {
                            if current_index.0 == 0 {
                                break;
                            }
                            current_index.0 -= 1;
                        }
                        Direction::FORWARD => {
                            if current_index.0 == board.num_rows() - 1 {
                                break;
                            }
                            current_index.0 += 1;
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
//1 == number
//dbg!()
//println!()
//matches!()
//assert!()

// for _num 0..n {}
pub enum Direction {
    UP,
    DOWN,
    FORWARD,
    BACKWARD,
    //TODO add more directions for diagonals
}
