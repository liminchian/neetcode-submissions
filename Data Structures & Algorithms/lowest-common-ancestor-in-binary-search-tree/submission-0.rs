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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root.clone()
        && let Some(q) = q
        && let Some(p) = p {
            let root_val = root.borrow().val;
            let p_val = p.borrow().val;
            let q_val = q.borrow().val;

            if p_val.max(q_val) < root_val {
                return Self::lowest_common_ancestor(
                    root.borrow().left.clone(),
                    Some(p),
                    Some(q)
                );
            } else if p_val.min(q_val) > root_val {
                return Self::lowest_common_ancestor(
                    root.borrow().right.clone(),
                    Some(p),
                    Some(q)
                );
            } else {
                return Some(root);
            }
        }
        root
    }
}
