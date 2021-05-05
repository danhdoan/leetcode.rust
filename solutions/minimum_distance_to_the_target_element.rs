///
/// Link: https://leetcode.com/problems/minimum-distance-to-the-target-element
///

// ============================================================================

struct Solution {
}

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, t: i32, s: i32) -> i32 {
        let mut i = s;
        let mut j = s;
        while i >= 0 || j < nums.len() as i32 {
            if i >= 0 && nums[i as usize] == t {
                return (i - s).abs();
            }

            if j < nums.len() as i32 && nums[j as usize] == t {
                return (j - s).abs();
            }
            i -= 1;
            j += 1;
        }
        0
    }
}

// ============================================================================

fn main() {
}

// ============================================================================

#[test]
fn test_1() {
    assert_eq!(Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::get_min_distance(vec![1], 1, 0), 0);
}

// ============================================================================

#[test]
fn test_3() {
    assert_eq!(Solution::get_min_distance(vec![1,1,1,1,1,1,1,1,1,1], 1, 0), 0);
}

// ============================================================================
