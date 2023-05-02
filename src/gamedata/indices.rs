pub fn inc_row((row, col): &(usize, usize), value: usize) -> (usize, usize) {
    (row + value as usize, *col)
}
pub fn inc_col((row, col): &(usize, usize), value: usize) -> (usize, usize) {
    (*row, col + value as usize)
}

pub fn dec_row((row, col): &(usize, usize), value: usize) -> (usize, usize) {
    (row - value as usize, *col)
}
pub fn dec_col((row, col): &(usize, usize), value: usize) -> (usize, usize) {
    (*row, col - value as usize)
}
pub fn inc_both((row, col): &(usize, usize), value: usize) -> (usize, usize) {
    (row + value, col + value)
}
pub fn dec_both((row, col): &(usize, usize), value: usize) -> (usize, usize) {
    (row - value, col - value)
}
//TODO get better names for these
pub fn inc_dec((row, col): &(usize, usize), value: usize) -> (usize, usize) {
    (row + value, col - value)
}
pub fn dec_inc((row, col): &(usize, usize), value: usize) -> (usize, usize) {
    (row - value, col + value)
}
