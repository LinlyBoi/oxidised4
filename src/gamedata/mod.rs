#[cfg(test)]
mod tests;
pub struct Board {
    p1_score: i32,
    p2_score: i32,
    columns: Vec<Vec<i32>>,
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
