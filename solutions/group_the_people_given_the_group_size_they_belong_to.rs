///
/// Link: https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/
///

// ============================================================================

use std::collections::HashMap;

struct Solution {
}

impl Solution {
    pub fn group_the_people(gr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, val) in gr.iter().enumerate() {
            if let Some(v) = hm.get_mut(&val) {
                v.push(i as i32);
            }
            else {
                hm.insert(*val, vec![i as i32]);
            }
        }
        
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for (size, lst) in &hm {
            for i in (0..lst.len()).step_by(*size as usize) {
                ans.push(lst[i..(i + *size as usize)].to_vec());
            }
        }
        
        ans
    }
}
