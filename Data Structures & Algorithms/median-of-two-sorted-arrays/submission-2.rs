impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        let mut l = 0;
        let mut r = a.len();
        let total = a.len() + b.len();
        let half = total.div_ceil(2);

        while l <= r {
            let i = (l + r) / 2;
            let j = half - i;

            let a_left = a.get(i - 1).unwrap_or(&i32::MIN);
            let b_right = b.get(j).unwrap_or(&i32::MAX);
    
            let b_left = b.get(j - 1).unwrap_or(&i32::MIN);
            let a_right = a.get(i).unwrap_or(&i32::MAX);

            if a_left <= b_right && b_left <= a_right {
                if total % 2 == 1 {
                    return *a_left.max(b_left) as f64;
                } else {
                    return (
                        a_left.max(b_left) + a_right.min(b_right)
                    ) as f64 / 2.0;
                }
            } else if a_left > b_right {
                r = i - 1;
            } else {
                l = i + 1;
            }
        }
        0.0
    }
}
