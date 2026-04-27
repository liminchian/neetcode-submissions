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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        let Some(p) = p else {
            return false;
        };
        let Some(q) = q else {
            return false;
        };
        let mut queue = VecDeque::new();
        queue.push_back((p, q));
        while let Some((p_node, q_node)) = queue.pop_front() {
            let p = p_node.borrow();
            let q = q_node.borrow();
            let p_l = p.left.as_ref().map(|rc| rc.borrow().val);
            let p_r = p.right.as_ref().map(|rc| rc.borrow().val);
            let q_l = q.left.as_ref().map(|rc|rc.borrow().val);
            let q_r = q.right.as_ref().map(|rc|rc.borrow().val);

            if p.val == q.val && p_l == q_l && p_r == q_r {
                if p_l.is_none() && p_r.is_none() {
                    break;
                }
                if p_l.is_some() {
                    queue.push_back((p.left.clone().unwrap(), q.left.clone().unwrap()));
                }
                if p_r.is_some() {
                    queue.push_back((p.right.clone().unwrap(), q.right.clone().unwrap()));
                }
            } else {
                return false;
            }
        }
        true
    }
}
