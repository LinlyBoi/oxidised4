mod heuristic;
mod indices;
mod score_checkers;
#[cfg(test)]
mod tests;

use array2d::Array2D;
pub use indices::*;

use self::score_checkers::scan;
#[derive(Clone)]
pub struct Board {
    red_score: i32,
    blu_score: i32,
    columns: Array2D<Disk>,
}

impl Default for Board {
    fn default() -> Self {
        let columns = Array2D::filled_with(Disk::EMPTY, 7, 6);

        Self {
            red_score: 0,
            blu_score: 0,
            columns,
        }
    }
}

impl Board {
    fn getscore(&self) -> (i32, i32) {
        (self.red_score, self.blu_score)
    }
    fn play(&mut self, disk: Disk, col: usize) -> bool {
        let column = &self.columns.as_columns()[col as usize];
        let empty = column.iter().filter(|&a| matches!(a, Disk::EMPTY)).count();
        dbg!(empty);
        let top = column.len() - empty;
        match self.columns.set(top, col as usize, disk) {
            Ok(_) => {
                self.score_check((top, col));
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
                Disk::EMPTY => return,
            },
            None => return,
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
}

#[derive(Copy, Clone, Debug)]
pub enum Disk {
    RED,
    BLU,
    EMPTY,
}
