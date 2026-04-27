use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mp: HashMap<&i32, usize> = nums.iter().enumerate().map(|(idx, num)| (num, idx)).collect();

        for (i, n) in nums.iter().enumerate() {
            let new_target = target - n;
            if let Some(j) = mp.get(&new_target) && i != *j {
                return vec![i as i32, *j as i32];
            }
        }
        vec![]
    }
}
