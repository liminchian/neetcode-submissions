use std::collections::HashSet;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut right = n as i32 - 1;

        while left <= right {
            let i = left + (right - left) / 2;
            let val = nums[i as usize];

            if val > target {
                right = i - 1;
            } else if val < target {
                left = i + 1;
            } else {
                return i as i32;
            }
        }
        -1
    }
}
