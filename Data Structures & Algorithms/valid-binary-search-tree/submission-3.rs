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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else {
            return false;
        };
        fn dfs(root: Rc<RefCell<TreeNode>>, min: i32, max: i32, is_valid: &mut bool) {
            let root_val = root.borrow().val;
            if root_val <= min || root_val >= max {
                *is_valid = false;
            }
            if let Some(l) = &root.borrow().left {
                dfs(l.clone(), min, root_val, is_valid);
            }
            if let Some(r) = &root.borrow().right {
                dfs(r.clone(), root_val, max, is_valid);
            }
        }
        let mut is_valid = true;
        dfs(root, std::i32::MIN, std::i32::MAX, &mut is_valid);
        is_valid
    }
}
