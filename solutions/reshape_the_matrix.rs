///
/// Link: https://leetcode.com/problems/reshape-the-matrix
///

// ============================================================================

struct Solution {}

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (r, c) = (r as usize, c as usize);
        let num = mat.len() * mat[0].len();
        if r * c != num{
            return mat;
        }
        
        let mut ans = vec![vec![0; c]; r];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                let coor = i * mat[0].len() + j;
                ans[coor / c][coor % c] = mat[i][j];
            }
        }
        
        ans
    }
}

// ============================================================================

fn main() {
}
