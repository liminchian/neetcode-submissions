// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            match root {
                None => {}
                Some(node) => {
                    dfs(&node.borrow().left, nums);
                    nums.push(node.borrow().val);
                    dfs(&node.borrow().right, nums);
                }
            }
        }
        let mut nums = vec![];
        dfs(&root, &mut nums);
        nums[k as usize - 1]
    }
}
