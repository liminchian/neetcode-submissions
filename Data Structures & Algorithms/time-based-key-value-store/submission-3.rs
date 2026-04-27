use std::collections::HashMap;

struct TimeMap(HashMap<String, HashMap<i32, String>>);

impl TimeMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.0.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(table) = self.0.get(&key) {
            if let Some(val) = table.get(&timestamp) {
                return val.clone();
            } else {
                let mut ts: Vec<i32> = table.keys().copied().collect();
                ts.sort();
                if ts[0] <= timestamp {
                    let mut l = 0;
                    let mut r = ts.len() as i32 - 1;
                    let mut prev = 0;
                    while l <= r {
                        let mid = l + (r - l) / 2;
                        let curr_t = ts[mid as usize];
                        if curr_t > timestamp {
                            r = mid - 1;
                        } else if curr_t <= timestamp {
                            prev = mid;
                            l = mid + 1;
                        }
                    }
                    return table.get(&ts[prev as usize]).unwrap_or(&"".to_string()).clone();
                }
            }
        }
        "".to_string()
    }
}
