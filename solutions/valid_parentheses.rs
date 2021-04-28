///
/// Link: https://leetcode.com/problems/valid-parentheses/
///

// ============================================================================

struct Solution {
}

impl Solution {
    fn is_match(u: char, v: char) -> bool {
        u == '(' && v == ')' || u == '[' && v == ']' || u == '{' && v == '}'
    }

    pub fn is_valid(s: String) -> bool {
        let mut vec: Vec<char> = Vec::new();
        let s_v: Vec<char> = s.chars().collect();

        for v in &s_v {
            if *v == '(' || *v == '[' || *v == '{' {
                vec.push(*v);
            }
            else if vec.len() > 0 {
                if Solution::is_match(vec[vec.len() - 1], *v) {
                    vec.pop();
                }
                else {
                    return false;
                }
            }
            else {
                return false;
            }
        }

        vec.len() == 0
    }
}

// ============================================================================

fn main() {
}

// ============================================================================

#[test]
fn test_1() {
    assert!(Solution::is_valid(String::from("()")));
}

// ============================================================================

#[test]
fn test_2() {
    assert!(Solution::is_valid(String::from("()[]{}")));
}

// ============================================================================

#[test]
fn test_3() {
    assert!(!Solution::is_valid(String::from("(]")));
}

// ============================================================================

#[test]
fn test_4() {
    assert!(!Solution::is_valid(String::from("([)])")));
}

// ============================================================================

#[test]
fn test_5() {
    assert!(Solution::is_valid(String::from("{[]}")));
}

// ============================================================================
