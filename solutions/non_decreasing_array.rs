///
/// Link: https://leetcode.com/problems/non-decreasing-array
///

// ============================================================================

struct Solution {
}

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut arr = nums.clone();
        arr.push(std::i32::MAX);

        let mut cnt = 0;

        for i in 0..arr.len()-1 {
            if arr[i] > arr[i+1] {
                cnt += 1;
                if i == 0 || arr[i-1] <= arr[i+1] {
                    arr[i] = arr[i+1];
                }
                else {
                    arr[i+1] = arr[i];
                }
            }
        }
        cnt <= 1
    }
}

// ============================================================================

fn main() {
}

// ============================================================================

#[test]
fn test_1() {
    assert!(Solution::check_possibility(vec![4,2,3]));
}

// ============================================================================

#[test]
fn test_2() {
    assert!(!Solution::check_possibility(vec![4,2,1]));
}

// ============================================================================
