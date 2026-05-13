use std::collections::HashSet;
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = candidates.clone();
        nums.sort();
        fn dfs(nums: &[i32], target: i32, i: usize, total: i32, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if total == target {
                res.push(subset.clone());
                return;
            }
            if i >= nums.len() || total > target {
                return;
            }
            subset.push(nums[i]);
            dfs(nums, target, i + 1, total + nums[i], subset, res);
            subset.pop();

            let mut j = i;
            while j + 1 < nums.len() && nums[j] == nums[j + 1] {
                j += 1;
            }
            dfs(nums, target, j + 1, total, subset, res);
        }
        dfs(&nums, target, 0, 0, &mut vec![], &mut res);
        res
    }
}
