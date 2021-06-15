///
/// Link: https://leetcode.com/problems/replace-all-digits-with-characters
///

// ============================================================================

struct Solution {}

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut cs: Vec<char> = s.chars().collect();

        for i in (1..s.len()).step_by(2) {
            cs[i] = ((cs[i-1] as u8 - 97 + cs[i] as u8 - 48) % 48 + 97) as char;
        }

        cs.iter().collect()
    }
}

// ============================================================================

fn main() {
}

// ============================================================================

#[test]
fn test_1() {
    assert_eq!(Solution::replace_digits(
            String::from("a1c1e1")), String::from("abcdef"));
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::replace_digits(
            String::from("a1b2c3d4e")), String::from("abbdcfdhe"));
}

// ============================================================================
