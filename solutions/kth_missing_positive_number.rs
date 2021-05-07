///
/// Link: https://leetcode.com/problems/kth-missing-positive-number
///

// ============================================================================

use std::cmp;

// ============================================================================

struct Solution {}

impl Solution {
    pub fn find_kth_positive(mut arr: Vec<i32>, mut k: i32) -> i32 {
        let (mut curr, mut idx) = (1, 0);
        while k > 0 {
            if idx < arr.len() && curr == arr[idx] {
                idx += 1;
            }
            else {
                k -= 1;
            }
            curr += 1;
        }
        curr - 1
    }
}

// ============================================================================
