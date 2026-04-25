impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let c = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
        if c.len() <= 1 {
            return true;
        }
        let c_bytes = c.as_bytes();
        let mut start = 0; 
        let mut end = c.len() - 1; 

        while start < end {
            if c_bytes[start] != c_bytes[end] {
                return false;
            }
            start += 1; 
            end -= 1;
    }
    return true; 

    }
}
