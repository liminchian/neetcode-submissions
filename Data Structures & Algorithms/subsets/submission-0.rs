impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![vec![]];

        for num in nums {
            let size = res.len();
            for i in 0..size {
                let mut subset = res[i].clone();
                subset.push(num);
                res.push(subset);
            }
        }

        res
    }
}
