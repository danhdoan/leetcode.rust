///
/// Link: https://leetcode.com/problems/count-good-meals
///

// ============================================================================

impl Solution {
    pub fn count_pairs(ds: Vec<i32>) -> i32 {
        let MOD = 1000000000 + 7;
        let mut cnts: Vec<u64> = vec![0; ((1 << 21) + 1) as usize];
        for x in &ds {
            cnts[*x as usize] += 1;
        }

        let mut ans: u64 = 0;
        for u in 0..cnts.len() {
            if cnts[u] == 0 {
                continue;
            }

            let mut pw: usize = 1;
            for _ in 0..22 {
                if u * 2 <= pw {
                    let v = pw - u;
                    if cnts[v] > 0 {
                        if u == v {
                            ans = (ans + ((cnts[u]-1) * cnts[u] / 2 % MOD)) % MOD;
                        }
                        else {
                            ans = (ans + cnts[u] * cnts[v] % MOD) % MOD;
                        }
                    }
                }
                pw *= 2;
            }
        }

        ans as i32
    }
}

// ============================================================================
