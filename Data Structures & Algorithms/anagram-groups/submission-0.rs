impl Solution {
    pub fn is_anagram(str1: String, str2: String) -> bool {
        let mut map = HashMap::new(); 
        for c in str1.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        for c in str2.chars() {
            if map.contains_key(&c) {
                let val = map.get_mut(&c).unwrap(); 
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
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans = Vec::new(); 
        let mut visited = vec![false; strs.len()];


        for i in 0..strs.len() {
            if visited[i] {
                continue;
            }
            let mut curr = Vec::new(); 
            curr.push(strs[i].clone());
            visited[i] = true;

            for j in 0..strs.len() {
                if j != i {
                    if !visited[j] && Self::is_anagram(strs[i].clone(), strs[j].clone()) {
                        curr.push(strs[j].clone());
                        visited[j] = true;
                    }
                }
            }
            ans.push(curr);
        }

        return ans; 

    }
}
