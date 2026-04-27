impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let length_encoded = strs.iter().map(|s| {
            s.len().to_string()
        }).collect::<Vec<String>>().join(",");
        let word_encoded = strs.join("");
        length_encoded + "#" + &word_encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        if let Some((lengths, strs)) = s.split_once("#") {
            let lengths = lengths.split(",").map(|s| s.parse::<usize>().expect("should be a number"));
            lengths.fold((vec![], 0), |(mut decoded, mut start_idx), length| {
                let end_idx = start_idx + length;
                decoded.push(strs[start_idx..end_idx].to_string());
                (decoded, end_idx)
            }).0
        } else {
            vec![]
        }
    }
}
