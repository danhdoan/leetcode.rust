///
/// Link: https://leetcode.com/problems/perfect-squares
///

// ============================================================================

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![-1; (n+1) as usize];
        dp[0] = 0;
        let sqr = (1..((n as f32).sqrt() as i32) + 1).map(|x| x*x).collect::<Vec<_>>();
        
        for v in 1..(n+1) {
            let v: usize = v as usize;
            for i in 0..sqr.len() {
                let s: usize = sqr[i] as usize;
                if v >= s && dp[v - s] != -1 {
                    if dp[v] == -1 || dp[v] > dp[v - s] + 1 {
                        dp[v] = dp[v - s] + 1;
                    }
                }
            }
        }
        
        dp[n as usize]
    }
}

// ============================================================================
