///
/// Link: https://leetcode.com/problems/isomorphic-strings
///

// ============================================================================

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut ds: HashMap<char, char> = HashMap::new();
        let mut dt: HashMap<char, char> = HashMap::new();

        for i in 0..s.len() {
            if ds.contains_key(&s[i]) && dt.contains_key(&t[i]) {
                if ds[&s[i]] != t[i] || dt[&t[i]] != s[i] {
                    return false;
                }
            }
            else if !ds.contains_key(&s[i]) && !dt.contains_key(&t[i]) {
                ds.insert(s[i], t[i]);
                dt.insert(t[i], s[i]);
            }
            else {
                return false;
            }
        }
        true
    }
}

// ============================================================================

#[test]
fn test_1() {
    assert!(Solution::is_isomorphic("egg".to_string(), "add".to_string()));
}

// ============================================================================

#[test]
fn test_2() {
    assert!(!Solution::is_isomorphic("foo".to_string(), "bar".to_string()));
}

// ============================================================================

#[test]
fn test_3() {
    assert!(Solution::is_isomorphic("title".to_string(), "paper".to_string()));
}

// ============================================================================
//
fn main() {
}
