///
/// Link: https://leetcode.com/problems/super-palindromes
///

// ============================================================================

use std::str::FromStr;
use std::cmp;
use std::collections::HashSet;

struct Solution {}

// ============================================================================

impl Solution {
    fn generate(x: usize, mut y: usize) -> u64 {
        let mut ans = x;
        while y > 0 {
            ans = ans * 10 + y % 10;
            y /= 10;
        }
        ans as u64
    }

    fn is_pal(mut x: u64) -> bool {
        let (x_, mut y) = (x, 0);
        while x > 0 {
            y = y * 10 + x % 10;
            x /= 10;
        }

        x_ == y
    }

    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let l = left.parse::<u64>().unwrap();
        let r = right.parse::<u64>().unwrap();
        let hi = cmp::min(100000, (r as f64).sqrt() as usize);

        let mut ans = HashSet::new();

        for i in 1..(hi + 1) {
            let x = Solution::generate(i, i);
            let mut x_s = x*x;
            if l <= x_s && x_s <= r && Solution::is_pal(x_s) {
                ans.insert(x_s);
            }

            let x = Solution::generate(i, i / 10);
            let mut x_s = x*x;
            if l <= x_s && x_s <= r && Solution::is_pal(x_s) {
                ans.insert(x_s);
            }
            if x_s > r {
                break;
            }
        }

        ans.len() as i32
    }
}

// ============================================================================
