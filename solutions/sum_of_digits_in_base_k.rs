///
/// Link: https://leetcode.com/problems/sum-of-digits-in-base-k/
///

// ============================================================================

struct Solution {
}

impl Solution {
    pub fn sum_base(mut n: i32, k: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            ans += n % k;
            n /= k;
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
    assert_eq!(Solution::sum_base(34, 6), 9);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::sum_base(10, 10), 1);
}

// ============================================================================