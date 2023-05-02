mod indices;
mod score_checkers;
#[cfg(test)]
mod tests;

use array2d::Array2D;
pub use indices::*;

use self::score_checkers::scan;
#[derive(Clone)]
pub struct Board {
    p1_score: i32,
    p2_score: i32,
    columns: Array2D<Disk>,
}

impl Default for Board {
    fn default() -> Self {
        let columns = Array2D::filled_with(Disk::EMPTY, 7, 6);

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
    fn play(&mut self, disk: Disk, col: i32) -> bool {
        let column = &self.columns.as_columns()[col as usize];
        let empty = column.iter().filter(|&a| matches!(a, Disk::EMPTY)).count();
        dbg!(empty);
        let top = column.len() - empty;
        match self.columns.set(top, col as usize, disk) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    fn score_check(&mut self, index: (usize, usize)) {
        unimplemented!()
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
// pub fn get_indices(
//     index: &(usize, usize),
//     op: fn(&(usize, usize), usize) -> (usize, usize),
//     values: Vec<usize>,
// ) -> Vec<(usize, usize)> {
//     let mut indices: Vec<(usize, usize)> = Vec::with_capacity(3);
//     for num in values {
//         indices.push(op(index, num));
//     }
//     indices
// }

#[derive(Copy, Clone, Debug)]
pub enum Disk {
    RED,
    BLU,
    EMPTY,
}
