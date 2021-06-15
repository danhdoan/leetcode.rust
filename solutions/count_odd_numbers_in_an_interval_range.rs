///
/// Link: https://leetcode.com/problems/count-odd-numbers-in-an-interval-range
///

// ============================================================================

use std::cmp;

// ============================================================================

struct Solution {}

impl Solution {
    pub fn count_odds(mut low: i32, mut high: i32) -> i32 {
        if low % 2 == 0 {
            low += 1;
        }
        if high % 2 == 0 {
            high -= 1;
        }
        (high - low) / 2 + 1
    }
}

// ============================================================================
