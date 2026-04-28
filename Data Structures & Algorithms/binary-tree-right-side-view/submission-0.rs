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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut view = vec![];
        let Some(root) = root else {
            return view;
        };
        let mut q = VecDeque::new();
        q.push_back(root);
        while let Some(node) = q.front() {
            view.push(node.borrow().val);
            let level_size = q.len();
            for _ in 0..level_size {
                let node = q.pop_front().unwrap();
                if let Some(r) = &node.borrow().right {
                    q.push_back(r.clone());
                }
                if let Some(l) = &node.borrow().left {
                    q.push_back(l.clone());
                }
            }
        }
        view
    }
}
