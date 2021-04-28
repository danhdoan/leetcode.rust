///
/// Link: https://leetcode.com/problems/two-sum/
///

use std::collections::HashMap;

// ============================================================================

struct Solution {
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, t: i32) -> Vec<i32> {
        let mut hm = HashMap::new();
        
        for i in 0..nums.len() {
            let y: i32 = t - nums[i];
            if hm.contains_key(&y) {
                return vec![*hm.get(&y).unwrap(), i as i32];
            }
            else {
                hm.insert(nums[i], i as i32);
            }
        }
        Vec::<i32>::new()
    }
}

// ============================================================================

fn main() {
}

// ============================================================================

#[test]
fn test_1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

// ============================================================================

#[test]
fn test_3() {
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
