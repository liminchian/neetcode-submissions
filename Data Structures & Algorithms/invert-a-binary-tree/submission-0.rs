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
use std::collections::VecDeque;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        let mut q = VecDeque::new();
        q.push_back(root.clone());
        while let Some(node) = q.pop_front() {
            let mut n = node.borrow_mut();
            let tmp = n.left.take();
            n.left = n.right.take();
            n.right = tmp;
            if let Some(left) = n.left.clone() {
                q.push_back(left);
            }
            if let Some(right) = n.right.clone() {
                q.push_back(right);
            }
        }
        Some(root)
    }
}
