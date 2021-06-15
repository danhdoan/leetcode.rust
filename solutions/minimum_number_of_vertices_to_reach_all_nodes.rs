///
/// Link: https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/
///

// ============================================================================

struct Solution {}

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut cnt: Vec<i32> = vec![0; n as usize];
        for e in &edges {
            cnt[e[1] as usize] += 1;
        }

        let mut ans: Vec<i32> = Vec::new();
        for i in 0..n as usize {
            if cnt[i] == 0 {
                ans.push(i as i32);
            }
        }
        ans
    }
}

// ============================================================================

#[test]
fn test_1() {
    assert_eq!(Solution::find_smallest_set_of_vertices(
            6, vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]]
    ), vec![0, 3]);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::find_smallest_set_of_vertices(
            5, vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]]
    ), vec![0, 2, 3]);
}

// ============================================================================
