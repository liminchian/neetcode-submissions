impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut max_seq = 0;
        let mut temp = 0;
        for i in 1..n {
            let delta = nums[i] - nums[i - 1];
            match delta {
                1 => {temp += 1;}
                delta if delta > 1 => {
                    temp += 1;
                    max_seq = max_seq.max(temp);
                    temp = 0;
                }
                _ => ()
            }
        }
        temp += 1;
        max_seq.max(temp)
    }
}
