///
/// Link: https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle
///

// ============================================================================

struct Solution {}

impl Solution {
    pub fn count_points(ps: Vec<Vec<i32>>, qs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for q in &qs {
            let x = q[0];
            let y = q[1];
            let r = q[2];
            let mut cnt = 0;
            for p in &ps {
                if (x - p[0])*(x - p[0]) + (y - p[1])*(y - p[1]) <= r*r {
                    cnt += 1;
                }
            }
            ans.push(cnt);
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
    let points: Vec<Vec<i32>> = vec![
        vec![1,3], vec![3,3], vec![5,3], vec![2,2]];
    let queries: Vec<Vec<i32>>= vec![vec![2,3,1],vec![4,3,1],vec![1,1,2]];
    assert_eq!(Solution::count_points(points, queries), vec![3, 2, 2]);
}

// ============================================================================

#[test]
fn test_2() {
    let points: Vec<Vec<i32>> = vec![
        vec![1,1], vec![2,2], vec![3,3], vec![4,4], vec![5,5]];
    let queries: Vec<Vec<i32>>= vec![
        vec![1,2,2],vec![2,2,2],vec![4,3,2],vec![4,3,3]];
    assert_eq!(Solution::count_points(points, queries), vec![2,3,2,4]);
}

// ============================================================================
