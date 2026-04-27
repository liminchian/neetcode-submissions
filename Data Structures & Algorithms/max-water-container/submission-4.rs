impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut i = 0;
        let mut j = n - 1;
        let mut max_area = heights[i].min(heights[j]) * (j - i) as i32;
        let max_height = heights[i + 1..j].iter().max().unwrap_or(&0);

        while heights[i].lt(max_height) {
            i += 1;
            let area = heights[i].min(heights[j]) * (j - i) as i32;
            max_area = max_area.max(area);
        }

        while heights[j].lt(max_height) {
            j -= 1;
            let area = heights[i].min(heights[j]) * (j - i) as i32;
            max_area = max_area.max(area);
        }

        max_area
    }
}
