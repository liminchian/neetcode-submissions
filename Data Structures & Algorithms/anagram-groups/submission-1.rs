use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut mp: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let sorted_s: String = {
                let mut s: Vec<char> = s.chars().collect();
                s.sort();
                s.iter().collect()
            };
            mp.entry(sorted_s).and_modify(|v| {
                v.push(s.clone());
            }).or_insert(vec![s]);
        }
        mp.into_values().collect()
    }
}
