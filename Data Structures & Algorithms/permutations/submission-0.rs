impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut pick = vec![false; nums.len()];
        
        fn backtrack(nums: &[i32], perm: &mut Vec<i32>, pick: &mut Vec<bool>, res: &mut Vec<Vec<i32>>) {
            if perm.len() == nums.len() {
                res.push(perm.clone());
                return;
            }
            for i in 0..nums.len() {
                if !pick[i] {
                    perm.push(nums[i]);
                    pick[i] = true;
                    backtrack(nums, perm, pick, res);
                    perm.pop();
                    pick[i] = false;
                }
            }
        }

        backtrack(&nums, &mut vec![], &mut pick, &mut res);
        res
    }
}
