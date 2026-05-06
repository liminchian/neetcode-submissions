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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_num: &mut i32) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let left_max = dfs(node.borrow().left.clone(), max_num).max(0);
                    let right_max = dfs(node.borrow().right.clone(), max_num).max(0);
                    let val = node.borrow().val;
                    *max_num = (*max_num).max(val + left_max + right_max);
                    val + left_max.max(right_max)
                }
            }
        }
        let mut val = root.as_ref().unwrap().borrow().val;
        dfs(root, &mut val);
        val
    }
}
