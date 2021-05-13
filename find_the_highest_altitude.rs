///
/// Link: https://leetcode.com/problems/find-the-highest-altitude
///

// ============================================================================

use std::cmp;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut alt: Vec<i32> = vec![0];

        let mut ans = 0;
        for (i, x) in gain.iter().enumerate() {
            alt.push(*x + alt[alt.len() - 1]);
            ans = cmp::max(ans, alt[alt.len() - 1]);
        }

        ans
    }
}

// ============================================================================
