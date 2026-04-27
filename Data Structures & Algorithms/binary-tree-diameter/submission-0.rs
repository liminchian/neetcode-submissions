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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };
        let mut stack = vec![root.clone()];
        let mut mp: HashMap<usize, (i32, i32)> = HashMap::new();

        while let Some(node_rc) = stack.last().cloned() {
            let node = node_rc.borrow();
            let left_ptr = node.left.as_ref().map(|rc| rc.as_ptr() as usize);
            let right_ptr = node.right.as_ref().map(|rc| rc.as_ptr() as usize);

            if let Some(l) = left_ptr && !mp.contains_key(&l) {
                stack.push(node.left.clone().unwrap());
                continue;
            }
            if let Some(r) = right_ptr && !mp.contains_key(&r) {
                stack.push(node.right.clone().unwrap());
                continue;
            }
            stack.pop();

            let (lh, ld) = left_ptr.and_then(|l| mp.get(&l)).unwrap_or(&(0, 0));
            let (rh, rd) = right_ptr.and_then(|r| mp.get(&r)).unwrap_or(&(0, 0));
            let height = 1 + lh.max(rh);
            let diameter = (lh + rh).max(*ld).max(*rd);
            mp.insert(node_rc.as_ptr() as usize, (height, diameter));
        }
        let key = root.as_ptr() as usize;
        mp.get(&key).unwrap_or(&(0, 0)).1
    }
}
