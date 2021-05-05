///
/// Link: https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree
///

// ============================================================================

struct Solution {}

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, lvl: usize, sums: &mut Vec<i32>) {
        match root {
            None => (),
            Some(root) => {
                if lvl == sums.len() {
                    sums.push(root.borrow().val);
                }
                else {
                    sums[lvl] += root.borrow().val;
                }
                
                Solution::dfs(root.borrow().left.as_ref(), lvl+1, sums);
                Solution::dfs(root.borrow().right.as_ref(), lvl+1, sums);
            }
        }
    }
    
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sums: Vec<i32> = Vec::new();
        Solution::dfs(root.as_ref(), 0, &mut sums);
        
        let (mut ans, mut max_s) = (0, std::i32::MIN);
        for i in 0..sums.len() {
            if sums[i] > max_s {
                max_s = sums[i];
                ans = i as i32;
            }
        }
        ans+1
    }
}

// ============================================================================
