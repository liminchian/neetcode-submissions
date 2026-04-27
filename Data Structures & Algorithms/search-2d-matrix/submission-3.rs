impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let mut row_index = matrix.len() - 1;
        for i in 1..matrix.len() {
            if matrix[i - 1][0] <= target && matrix[i][0] > target {
                row_index = i - 1;
            }
        }
        if matrix[0].len() == 1 {
            return matrix[row_index][0] == target;
        }
        let mut l = 0;
        let mut r = matrix[0].len() as i32 - 1;
        while l <= r {
            let i = l + (r - l) / 2;
            let val = matrix[row_index][i as usize];
            if target < val {
                r = i - 1;
            } else if target > val {
                l = i + 1;
            } else {
                return true;
            }
        }
        false
    }
}
