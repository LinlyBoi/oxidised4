pub mod algorithms;
mod heuristic;
mod indices;
mod score_checkers;
#[cfg(test)]
mod tests;

use array2d::Array2D;
pub use indices::*;

#[derive(Clone)]
pub struct Board {
    p1_score: i32,
    p2_score: i32,
    columns: Array2D<Disk>,
    pub last_move: i32,
}

impl Default for Board {
    fn default() -> Self {
        let columns = Array2D::filled_with(Disk::EMPTY, 6, 7);

        Self {
            p1_score: 0,
            p2_score: 0,
            columns,
            last_move: 0,
        }
    }
}

impl Board {
    pub fn getscore(&self) -> (i32, i32) {
        (self.p1_score, self.p2_score)
    }
    pub fn play(&mut self, disk: Disk, col: usize) -> bool {
        if self.game_over() {
            return false;
        }
        let column = &self.columns.as_columns()[col];
        let empty = column.iter().filter(|&a| matches!(a, Disk::EMPTY)).count();
        // dbg!(empty);
        let top = column.len() - empty;
        match self.columns.set(top, col, disk) {
            Ok(_) => {
                self.score_check((top, col));
                self.last_move = col as i32;
                //dbg!(self.p1_score, self.p2_score);
                true
            }
            Err(_) => false,
        }
    }
    fn score_check(&mut self, index: (usize, usize)) {
        use score_checkers::*;
        let moves = get_legal_moves(
            &index,
            (self.columns.num_rows(), self.columns.num_columns()),
        );
        match self.columns.get(index.0, index.1) {
            Some(disk) => match disk {
                Disk::P1 => {
                    for _move in moves {
                        let mut consecutive = scan(&self.columns, &index, _move.clone(), 4, *disk);
                        if consecutive < 4 {
                            consecutive += scan(
                                &self.columns,
                                &index,
                                flip_direction(_move.clone()),
                                4 - consecutive,
                                *disk,
                            ) - 1;
                        }
                        if consecutive == 4 {
                            self.p1_score += 1
                        }
                    }
                }
                Disk::P2 => {
                    for _move in moves {
                        let mut consecutive = scan(&self.columns, &index, _move.clone(), 4, *disk);
                        if consecutive < 4 {
                            consecutive += scan(
                                &self.columns,
                                &index,
                                flip_direction(_move.clone()),
                                4 - consecutive,
                                *disk,
                            ) - 1;
                        }
                        if consecutive == 4 {
                            self.p2_score += 1
                        }
                    }
                }
                Disk::EMPTY => (),
            },
            None => (),
        }
    }
    pub fn game_over(&self) -> bool {
        self.columns
            .as_row_major()
            .iter()
            .filter(|&d| matches!(d, &Disk::EMPTY))
            .count()
            == 0
    }
    fn get_children(&self, disk: Disk) -> Vec<Board> {
        let mut children: Vec<Board> = vec![];
        for column in 0..self.columns.num_columns() {
            let mut child = self.clone();
            match child.play(disk, column) {
                true => children.push(child),
                false => {}
            }
        }
        children
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Disk {
    P1,
    P2,
    EMPTY,
}
pub fn flip_disk(disk: Disk) -> Disk {
    match disk {
        Disk::P1 => Disk::P2,
        Disk::P2 => Disk::P1,
        Disk::EMPTY => Disk::EMPTY, //why..just why
    }
}
