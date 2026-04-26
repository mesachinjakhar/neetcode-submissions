use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut map: HashMap<u8, i32> = HashMap::new();
        let mut left = 0usize;
        let mut max_freq = 0i32;
        let mut ans = 0i32;

        for right in 0..n {
            let count = map.entry(bytes[right]).or_insert(0);
            *count += 1;

            max_freq = max_freq.max(*count);

            while (right - left + 1) as i32 - max_freq > k {
                if let Some(v) = map.get_mut(&bytes[left]) {
                    *v -= 1;
                }
                left += 1;
            }

            ans = ans.max((right - left + 1) as i32);
        }

        ans
    }
}