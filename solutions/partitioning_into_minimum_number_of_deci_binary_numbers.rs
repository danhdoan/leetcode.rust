///
/// Link: https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
///

// ============================================================================

struct Solution {}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().max().unwrap().to_digit(10).unwrap() as i32
    }
}


// ============================================================================

fn main() {
    
}

// ============================================================================

#[test]
fn test_1() {
    assert_eq!(Solution::min_partitions(String::from("32")), 3);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::min_partitions(String::from("82734")), 8);
}

// ============================================================================


#[test]
fn test_3() {
    assert_eq!(Solution::min_partitions(
            String::from("27346209830709182346")), 9);
}

// ============================================================================
