///
/// Link: https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram
///

// ============================================================================

use std::cmp;

// ============================================================================

struct Solution {}

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut cnts: Vec<i32> = vec![0; 26];
        let mut cntt: Vec<i32> = vec![0; 26];
        
        let cs = s.as_bytes();
        let ct = t.as_bytes();
        for i in 0..s.len() {
            cnts[(cs[i] - 97) as usize] += 1;
            cntt[(ct[i] - 97) as usize] += 1;
        }
        
        let mut cnt = 0;
        for i in 0..26 {
            cnt += cmp::min(cnts[i], cntt[i]);
        }
        
        s.len() as i32 - cnt
    }
}

// ============================================================================

#[test]
fn test_1() {
    assert_eq!(Solution::min_steps(
            String::from("bab"), String::from("aba")), 1);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::min_steps(
            String::from("leetcode"), String::from("practice")), 5);
}

// ============================================================================

#[test]
fn test_3() {
    assert_eq!(Solution::min_steps(
            String::from("anagram"), String::from("mangaar")), 0);
}

// ============================================================================

#[test]
fn test_4() {
    assert_eq!(Solution::min_steps(
            String::from("xxyyzz"), String::from("xxyyzz")), 0);
}

// ============================================================================

#[test]
fn test_5() {
    assert_eq!(Solution::min_steps(
            String::from("friend"), String::from("family")), 4);
}

// ============================================================================
