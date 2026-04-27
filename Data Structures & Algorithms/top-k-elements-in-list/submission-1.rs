use std::collections::HashMap;
use std::ops::AddAssign;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut mp: HashMap<i32, usize> = HashMap::new();
        for num in nums {
            mp.entry(num).or_insert(0).add_assign(1);
        }
        let mut counts: Vec<(i32, usize)> = mp.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        counts.iter().take(k as usize).map(|(num, _)| *num).collect()
    }
}
