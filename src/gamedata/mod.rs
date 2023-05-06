mod algorithms;
mod heuristic;
mod indices;
mod score_checkers;
#[cfg(test)]
mod tests;

use array2d::Array2D;
pub use indices::*;

#[derive(Clone)]
pub struct Board {
    red_score: i32,
    blu_score: i32,
    columns: Array2D<Disk>,
    last_move: usize,
}

impl Default for Board {
    fn default() -> Self {
        let columns = Array2D::filled_with(Disk::EMPTY, 6, 7);

        Self {
            red_score: 0,
            blu_score: 0,
            columns,
            last_move: 0,
        }
    }
}

impl Board {
    fn getscore(&self) -> (i32, i32) {
        (self.red_score, self.blu_score)
    }
    fn play(&mut self, disk: Disk, col: usize) -> bool {
        let column = &self.columns.as_columns()[col];
        let empty = column.iter().filter(|&a| matches!(a, Disk::EMPTY)).count();
        // dbg!(empty);
        let top = column.len() - empty;
        match self.columns.set(top, col, disk) {
            Ok(_) => {
                self.score_check((top, col));
                self.last_move = col;
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
                Disk::RED => {
                    for _move in moves {
                        let mut consecutive = scan(&self.columns, &index, _move.clone(), 4);
                        if consecutive < 4 {
                            consecutive += scan(
                                &self.columns,
                                &index,
                                flip_direction(_move),
                                4 - consecutive + 1,
                            )
                        }
                        if consecutive - 1 == 4 {
                            self.red_score += 1
                        }
                    }
                }
                Disk::BLU => {
                    for _move in moves {
                        let mut consecutive = scan(&self.columns, &index, _move.clone(), 4);
                        if consecutive < 4 {
                            consecutive += scan(
                                &self.columns,
                                &index,
                                flip_direction(_move),
                                4 - consecutive + 1,
                            )
                        }
                        if consecutive - 1 == 4 {
                            self.blu_score += 1
                        }
                    }
                }
                Disk::EMPTY => (),
            },
            None => (),
        }
    }
    fn game_over(&self) -> bool {
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
    RED,
    BLU,
    EMPTY,
}
pub fn flip_disk(disk: Disk) -> Disk {
    match disk {
        Disk::RED => Disk::BLU,
        Disk::BLU => Disk::RED,
        Disk::EMPTY => Disk::EMPTY, //why..just why
    }
}
