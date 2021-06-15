///
/// Link: https://leetcode.com/problems/count-primes
///

// ============================================================================

struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut vs: Vec<bool> = vec![true; n as usize];

        let mut i = 2;
        while i*2 < n {
            if vs[i as usize] {
                let mut j = 2*i;
                while j < n {
                    vs[j as usize] = false;
                    j += i;
                }
            }
            i += 1;
        }

        let mut ans = 0;
        for i in 2..(n as usize) {
            ans += vs[i] as i32;
        }
        ans
    }
}

// ============================================================================
