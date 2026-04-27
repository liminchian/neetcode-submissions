impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let n = numbers.len();
        let mut i = 0;
        let mut j = i + 1;
        loop {
            if i > n - 2 {
                break;
            }
            if j > n - 1 {
                i += 1;
                j = i + 1;
            }
            if target == (numbers[i] + numbers[j]) {
                return vec![i as i32 + 1, j as i32 + 1];
            }
            j += 1;
        }
        vec![0, 0]
    }
}
