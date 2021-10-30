///
/// Link: https://leetcode.com/problems/count-number-of-pairs-with-absolute-difference-k
///

// ============================================================================

struct Solution {
}

use std::collections::HashMap;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        
        for x in nums.iter() {
            if let Some(v) = hm.get_mut(x) {
                *v += 1;
            }
            else {
                hm.insert(*x, 1);
            }
        }
        
        let mut ans = 0;
        for (x, c) in &hm {
            if let Some(v) = &hm.get(&(x - k)) {
                ans += *v * c;
            }
        }
        
        ans
    }
}
