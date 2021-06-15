///
/// Link: https://leetcode.com/problems/maximum-units-on-a-truck/
///

// ============================================================================

use std::cmp;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut bbox = box_types.clone();
        bbox.sort_by(|a, b| b[1].cmp(&a[1]));
        
        let mut ans = 0;
        let mut size = truck_size;
        for i in 0..bbox.len() {
            let cnt = cmp::min(bbox[i][0], size);
            ans += cnt * bbox[i][1];
            size -= cnt;
            if size == 0 {
                break;
            }
        }
        
        ans
    }
}

// ============================================================================

fn main() {
}

// ============================================================================
