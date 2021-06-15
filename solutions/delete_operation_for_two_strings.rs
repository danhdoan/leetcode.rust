///
/// Link: https://leetcode.com/problems/delete-operation-for-two-strings
///

// ============================================================================

use std::cmp;

// ============================================================================

struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let s = word1.as_bytes();
        let t = word2.as_bytes();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; t.len()+1]; s.len()+1];

        for i in 1..dp.len() {
            for j in 1..dp[0].len() {
                dp[i][j] = match s[i-1] == t[j-1] {
                    true => dp[i-1][j-1] + 1,
                    _ => cmp::max(dp[i][j-1], dp[i-1][j])
                };
            }
        }

        (s.len() + t.len()) as i32 - 2 * dp[s.len()][t.len()]
    }
}

// ============================================================================
