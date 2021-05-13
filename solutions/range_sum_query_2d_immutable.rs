///
/// Link: https://leetcode.com/problems/range-sum-query-2d-immutable
///

// ============================================================================

struct NumMatrix {
    dp: Vec<Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(arr: Vec<Vec<i32>>) -> Self {
        let dp = NumMatrix::_build(arr);

        Self {
            dp: dp,
        }
    }

    fn _build(arr: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = arr.len() + 1;
        let m = arr[0].len() + 1;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];
        for i in 0..arr.len() {
            for j in 0..arr[0].len() {
                dp[i+1][j+1] = arr[i][j] + dp[i+1][j] + dp[i][j+1] - dp[i][j];
            }
        }
        dp
    }

    fn sum_region(&self, r1: i32, c1: i32, r2: i32, c2: i32) -> i32 {
        self.dp[(r2+1) as usize][(c2+1) as usize] -
        self.dp[(r2+1) as usize][c1 as usize] -
        self.dp[r1 as usize][(c2+1) as usize] +
        self.dp[r1 as usize][c1 as usize]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

// ============================================================================
