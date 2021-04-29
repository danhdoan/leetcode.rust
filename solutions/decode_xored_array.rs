///
/// Link: https://leetcode.com/problems/decode-xored-array/
///

// ============================================================================

struct Solution {
}

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![first];
        for i in 0..encoded.len() {
            ans.push(encoded[i] ^ ans[ans.len() - 1]);
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
    assert_eq!(Solution::decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
}

// ============================================================================
