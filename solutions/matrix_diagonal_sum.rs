///
/// Link: https://leetcode.com/problems/matrix-diagonal-sum
///

// ============================================================================

struct Solution {
}

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for i in 0..mat.len() {
            ans += mat[i][i] + mat[i][mat.len() - i - 1];
        }
        if mat.len() % 2 == 1 {
            ans -= mat[mat.len()/2][mat.len()/2];
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
    let mat:Vec<Vec<i32>> = vec![
        vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    assert_eq!(Solution::diagonal_sum(mat), 25);
}

// ============================================================================

#[test]
fn test_2() {
    let mat:Vec<Vec<i32>> = vec![
        vec![1, 1, 1, 1], 
        vec![1, 1, 1, 1], 
        vec![1, 1, 1, 1], 
        vec![1, 1, 1, 1]];
    assert_eq!(Solution::diagonal_sum(mat), 8);
}

// ============================================================================

#[test]
fn test_3() {
    let mat:Vec<Vec<i32>> = vec![vec![5]];
    assert_eq!(Solution::diagonal_sum(mat), 5);
}

// ============================================================================
