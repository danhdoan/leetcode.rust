///
/// Link: https://leetcode.com/problems/powerful-integers
///

// ============================================================================

use std::collections::HashSet;

// ============================================================================

struct Solution {
}

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut b1 = 0;
        let mut p = 1;
        while x > 1 && p <= bound {
            b1 += 1;
            p *= x;
        }
        let mut b2 = 0;
        let mut p = 1;
        while y > 1 && p <= bound {
            b2 += 1;
            p *= y;
        }

        let mut ans_s: HashSet<i32> = HashSet::new();
        let mut p1 = 1;
        for i in 0..(b1 + 1) {
            let mut p2 = 1;
            for j in 0..(b2 + 1) {
                if p1 + p2 <= bound {
                    ans_s.insert(p1 + p2);
                    p2 *= y;
                }
                else {
                    break;
                }
            }
            p1 *= x;
        }

        let mut ans = vec![];
        for x in &ans_s {
            ans.push(*x);
        }
        ans
    }
}

// ============================================================================

fn main() {
}

// ============================================================================
