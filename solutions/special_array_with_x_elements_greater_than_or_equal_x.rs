///
/// Link: https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x
///

// ============================================================================

struct Solution {}

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in (0..nums.len()).rev() {
            let cnt = (nums.len() - i) as i32;
            if nums[i] == cnt && (i == 0 || i > 0 && nums[i-1] < cnt) {
                return nums[i];
            }
            else if i > 0 && nums[i-1] < cnt && cnt < nums[i] {
                return cnt;
            }
        }
        
        if (nums.len() as i32) < nums[0] {nums.len() as i32} else {-1}
    }
}

// ============================================================================
