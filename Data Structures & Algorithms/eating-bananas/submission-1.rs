impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut min_rate = *piles.iter().max().unwrap_or(&std::i32::MAX);
        let mut right = min_rate;
        let mut left = 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let temp_h: i32 = piles.iter().map(|p| {
                if p % mid == 0 {
                    p / mid
                } else {
                    p / mid + 1
                }
            }).sum();
            if temp_h <= h {
                right = mid - 1;
                min_rate = min_rate.min(mid);
            } else {
                left = mid + 1;
            }
        }
        min_rate
    }
}
