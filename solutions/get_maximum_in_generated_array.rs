///
/// Link: https://leetcode.com/problems/get-maximum-in-generated-array
///

// ============================================================================

use std::cmp;

// ============================================================================

struct Solution {}

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut vals: Vec<i32> = vec![0; n as usize + 1];

        vals[1] = 1;
        let mut ans = 1;
        for i in 2..vals.len() {
            vals[i] = match i % 2 {
                0 => vals[i / 2],
                1 => vals[i / 2] + vals[i / 2 + 1],
                _ => -1
            };

            ans = cmp::max(ans, vals[i]);
        }
        ans
    }
}

// ============================================================================
