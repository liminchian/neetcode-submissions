impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix = vec![0; n];
        let mut suffix = vec![0; n];
        let mut result = vec![0; n];

        prefix[0] = 1;
        suffix[n - 1] = 1;

        for i in 1..n {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }
        for i in (0..n - 1).rev() {
            suffix[i] = suffix[i + 1] * nums[i + 1];
        }
        for i in 0..n {
            result[i] = prefix[i] * suffix[i];
        }

        result
    }
}
