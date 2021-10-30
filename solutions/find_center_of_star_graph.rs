///
/// Link: https://leetcode.com/problems/find-center-of-star-graph
///

// ============================================================================

struct Solution {
}

impl Solution {
    pub fn find_center(E: Vec<Vec<i32>>) -> i32 {
         match E[0][0] == E[1][0] || E[0][0] ==E[1][1] {
            true => E[0][0],
            _ => E[0][1]
        }
    }
}
