mod score_checkers;
#[cfg(test)]
mod tests;

use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use array2d::Array2D;

use self::score_checkers::{one_direction, two_direction};
#[derive(Clone)]
pub struct Board {
    p1_score: i32,
    p2_score: i32,
    columns: Vec<Vec<Disk>>,
}

impl Default for Board {
    fn default() -> Self {
        let mut columns = Vec::with_capacity(6);
        for _num in 0..7 {
            columns.push(Vec::with_capacity(7))
        }
        Self {
            p1_score: 0,
            p2_score: 0,
            columns,
        }
    }
}

impl Board {
    fn getscore(&self) -> (i32, i32) {
        (self.p1_score, self.p2_score)
    }
    fn play(&mut self, disk: Disk, col: i32) {
        match self.columns.get_mut(col as usize) {
            Some(column) => {
                if column.len() != column.capacity() {
                    column.push(disk)
                } else {
                    println!("Column full")
                }
            }
            None => println!("column not exist"), //TODO properly handle this
        }
    }
    fn score_check(&mut self, index: (usize, usize)) {
        let _board = Array2D::from_columns(&self.columns);
        match _board {
            Ok(board) => {
                // thread::scope(|scope| {
                //     let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
                //     let one = scope.spawn(|| tx.clone().send(one_direction(&board, &index)));
                //     let two = scope.spawn(|| two_direction(&board, &index));
                //     one.join();
                //     two.join();
                // }); //ignore this is threading for later TODO
            }
            Err(_err) => println!("Error parsing the thing"),
        }
    }
}
#[derive(Clone)]
pub enum Disk {
    RED,
    BLUE,
}
