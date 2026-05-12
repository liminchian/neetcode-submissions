impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut subset = vec![];
        let mut res = vec![];

        fn dfs(nums: &[i32], target: i32, i: usize, total: i32, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if target == total {
                res.push(subset.clone());
                return;
            }
            if i >= nums.len() || total > target {
                return;
            }
            subset.push(nums[i]);
            dfs(nums, target, i, total + nums[i], subset, res);
            subset.pop();
            dfs(nums, target, i + 1, total, subset, res);
        }
        dfs(&nums, target, 0, 0, &mut subset, &mut res);
        res
    }
}
