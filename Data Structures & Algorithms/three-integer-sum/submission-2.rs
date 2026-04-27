use std::collections::HashSet;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut results = HashSet::new();

        for i in 0..n {
            let new_target = 0 - nums[i];
            let mut j = i + 1;
            let mut k = n - 1;
            while k > j {
                let sum = nums[j] + nums[k];
                if sum.eq(&new_target) {
                    results.insert(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                } else if sum.lt(&new_target) {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        if results.is_empty() {
            vec![]
        } else {
            results.into_iter().collect()
        }
    }
}
