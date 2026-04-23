use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1; 
        }
        for c in t.chars() {
            if let Some(val) = map.get_mut(&c) {
                *val -= 1;

                if *val == 0 {
                    map.remove(&c);
                }
            } else {
                return false; 
            }
        }

        map.is_empty()
    }
}
