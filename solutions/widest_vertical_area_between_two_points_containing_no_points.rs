///
/// Link: https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points
///

// ============================================================================

struct Solution {
}

use std::cmp;

impl Solution {
    pub fn max_width_of_vertical_area(pts: Vec<Vec<i32>>) -> i32 {
        let mut pts = pts;
        pts.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut ans = -1;
        for i in 1..pts.len() {
            ans = cmp::max(ans, pts[i][0] - pts[i-1][0]);
        }
        
        ans
    }
}
