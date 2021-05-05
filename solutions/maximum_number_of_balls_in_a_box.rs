///
/// Link: https://leetcode.com/problems/maximum-number-of-balls-in-a-box
///

use std::cmp;

// ============================================================================

struct Solution {
}

impl Solution {
    fn f(mut x: i32) -> i32 {
        let mut ans = 0;
        while x > 0 {
            ans += x % 10;
            x /= 10;
        }
        ans
    }
    
    pub fn count_balls(lo: i32, hi: i32) -> i32 {
        let mut cnt: Vec<i32> = vec![0; 46];
        let mut ans= 0;
        for x in lo..hi+1 {
            let val = Solution::f(x) as usize;
            cnt[val as usize] += 1;
            ans = cmp::max(ans, cnt[val]);
        }
        ans
    }
}

// ============================================================================

fn main() {
}

// ============================================================================

#[test]
fn test_1() {
    assert_eq!(Solution::count_balls(1, 10), 2);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::count_balls(5, 15), 2);
}

// ============================================================================

#[test]
fn test_3() {
    assert_eq!(Solution::count_balls(19, 28), 2);
}

// ============================================================================
