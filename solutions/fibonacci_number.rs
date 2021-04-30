///
/// Link: https://leetcode.com/problems/fibonacci-number
///

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {n} else {Solution::fib(n-1) + Solution::fib(n-2)}
    }
}
