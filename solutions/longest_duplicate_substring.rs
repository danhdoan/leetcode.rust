///
/// Link: https://leetcode.com/problems/longest-duplicate-substring
///

// ============================================================================

use std::collections::HashMap;

struct Solution {
}

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut m: HashMap<char, Vec<usize>> = HashMap::new();
        let n = s.len();
        
        for (i, c) in s.iter().enumerate() {
            if let Some(v) = m.get_mut(c) {
                v.push(i);
            }
            else {
                m.insert(*c, vec![i]);
            }
        }
        
        let mut idx = 0;
        let mut max_l: usize = 0;
        for (i, c) in s.iter().enumerate() {
            if i + max_l >= n {
                break;
            }
            
            for j in m.get(c).unwrap().iter() {
                if *j <= i || *j + max_l >= n {
                    continue;
                }
                
                let mut k = 0;
                while *j + k < n && s[i + k] == s[*j + k] {
                    k += 1;
                }
                
                if k > max_l {
                    idx = i;
                    max_l = k as usize;
                }
            }
        }
                
        if max_l == 0 {"".to_string()} else {s[idx..idx+max_l].iter().collect::<String>()}
    }
}

