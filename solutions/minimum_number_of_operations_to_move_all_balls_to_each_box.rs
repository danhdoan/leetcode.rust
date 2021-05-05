///
/// Link: https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/
///

// ============================================================================

struct Solution {}

impl Solution {
    pub fn min_operations(b: String) -> Vec<i32> {
        let cs: Vec<char> = b.chars().collect();
        let mut ans: Vec<i32> = vec![0; b.len()];

        let mut curr = 0;
        let mut total = 0;
        for i in 0..b.len() {
            ans[i] += total;
            curr += if cs[i] == '1' {1} else {0};
            total += curr;
        }

        curr = 0;
        total = 0;
        for i in (0..b.len()).rev() {
            ans[i] += total;
            curr += if cs[i] == '1' {1} else {0};
            total += curr;
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
    assert_eq!(Solution::min_operations(String::from("110")), vec![1, 1, 3]);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::min_operations(
            String::from("001011")), vec![11, 8, 5, 4, 3, 4]);
}

// ============================================================================
