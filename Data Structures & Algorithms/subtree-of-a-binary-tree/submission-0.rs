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
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut is_subtree = false;
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>, is_subtree: &mut bool) {
            if let Some(root) = root && let Some(sub_root) = sub_root {
                if root == sub_root {
                    *is_subtree = true;
                    dfs(root.borrow().left.clone(), sub_root.borrow().left.clone(), is_subtree);
                    dfs(root.borrow().right.clone(), sub_root.borrow().right.clone(), is_subtree);
                } else {
                    dfs(root.borrow().left.clone(), Some(sub_root.clone()), is_subtree);
                    dfs(root.borrow().right.clone(), Some(sub_root), is_subtree);
                }
            }
        }
        dfs(root, sub_root, &mut is_subtree);
        is_subtree
    }
}
