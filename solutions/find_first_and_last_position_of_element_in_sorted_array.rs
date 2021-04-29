///
/// Link: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array
///

// ============================================================================

struct Solution {
}

impl Solution {
    fn bs_left(nums: &Vec<i32>, t: i32) -> i32 {
        let mut ans = -1 as i32;
        let mut lo = 0 as i32;
        let mut hi = (nums.len() - 1) as i32;
        while lo <= hi {
            let md = lo + (hi - lo) / 2;
            if nums[md as usize] < t {
                lo = md + 1;
            }
            else {
                if nums[md as usize] == t {
                    ans = md as i32;
                }
                hi = md - 1;
            }
        }
        ans
    }

    fn bs_right(nums: &Vec<i32>, t: i32) -> i32 {
        let mut ans = -1 as i32;
        let mut lo = 0 as i32;
        let mut hi = (nums.len() - 1) as i32;
        while lo <= hi {
            let md = lo + (hi - lo) / 2;
            if nums[md as usize] > t {
                hi = md - 1;
            }
            else {
                if nums[md as usize] == t {
                    ans = md as i32;
                }
                lo = md + 1;
            }
        }
        ans
    }

    pub fn search_range(nums: Vec<i32>, t: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        vec![Solution::bs_left(&nums, t), Solution::bs_right(&nums, t)]
    }
}

// ============================================================================

fn main() {
}

// ============================================================================

#[test]
fn test_1() {
    assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 8), vec![3, 4]);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 6), vec![-1, -1]);
}

// ============================================================================

#[test]
fn test_3() {
    assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
}

// ============================================================================
