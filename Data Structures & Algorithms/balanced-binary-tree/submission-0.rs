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
use std::collections::HashMap;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else {
            return true;
        };
        let mut stack = vec![root];
        let mut heights = HashMap::new();

        while let Some(node) = stack.last().cloned() {
            let n = node.borrow();
            let addr = node.as_ref().as_ptr() as usize;
            let left_addr = n.left.as_ref().map(|rc| rc.as_ptr() as usize);
            let right_addr = n.right.as_ref().map(|rc| rc.as_ptr() as usize);

            if let Some(l) = left_addr && !heights.contains_key(&l) {
                stack.push(n.left.clone().unwrap());
                continue;
            }
            if let Some(r) = right_addr && !heights.contains_key(&r) {
                stack.push(n.right.clone().unwrap());
                continue;
            }
            let left_height = *left_addr.and_then(|addr| heights.get(&addr)).unwrap_or(&0);
            let right_height = *right_addr.and_then(|addr| heights.get(&addr)).unwrap_or(&0);
            if (left_height as i32 - right_height as i32).abs() > 1 {
                return false;
            }
            heights.insert(addr, 1 + left_height.max(right_height));
            stack.pop();
        }
        true
    }
}
