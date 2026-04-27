use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_valids: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut column_valids: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut square_valids: HashMap<usize, HashSet<char>> = HashMap::new();

        for (i, row) in board.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if c.eq(&'.') {
                    continue;
                }
                if !(
                    row_valids.entry(i).or_default().insert(*c)
                    && column_valids.entry(j).or_default().insert(*c)
                    && square_valids.entry(square_num(i, j)).or_default().insert(*c)
                ) {
                    return false;
                }
            }
        }
        true
    }
}
pub fn square_num(i: usize, j: usize) -> usize {
    (i / 3) * 3 + (j / 3)
}
