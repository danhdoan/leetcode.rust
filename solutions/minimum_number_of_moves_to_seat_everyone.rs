///
/// Link: https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone
///

// ============================================================================

struct Solution {
}

impl Solution {
    pub fn min_moves_to_seat(sts: Vec<i32>, stds: Vec<i32>) -> i32 {
        let mut sts = sts;
        let mut stds = stds;
        sts.sort(); stds.sort();
        
        let mut ans = 0;
        for i in 0..sts.len() {
            ans += (sts[i] - stds[i]).abs();
        }
        ans
    }
}
