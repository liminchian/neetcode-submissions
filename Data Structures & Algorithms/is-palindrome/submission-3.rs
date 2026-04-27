impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.to_lowercase().chars().filter(|c| {
            c.is_ascii_digit() || c.is_ascii_alphabetic()
        }).collect();
        if chars.is_empty() {
            return true;
        }
        let mut i = 0;
        let mut j = chars.len() - 1;
        loop {
            if i >= j {
                break;
            }
            if chars[i].ne(&chars[j]) {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
